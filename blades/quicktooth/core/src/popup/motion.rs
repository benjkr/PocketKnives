//! The header's motion while the menu is open (the approved design "2 · B1 alive"): springs for
//! the lid, each bud's way out of the case and the case's fade; the lid snaps shut with a bump;
//! the buds hop out one after the other and bob a whole pixel while out; the status light blinks
//! amber for a while after the case closes. Time comes from the caller (the host's clock).
use crate::battery::Pods;
use crate::math::{PI, cos, max, min};

/// The lid's angle, open.
pub const OPEN: f64 = 102.0;
/// The case's opacity once both buds are out (it fades and sinks out of the way).
pub const FADED: f64 = 0.35;
/// How long a bud's bob takes (up and down), ms; the right one is half a period behind.
const BOB_MS: f64 = 1517.0;
/// The light blinks amber for this long after the case closes, this fast.
const BLINK_MS: f64 = 5200.0;
const BLINK_STEP_MS: f64 = 570.0;
/// The lid's angle below which the buds wait (they can't pass a lid that's in the way).
const LID_CLEAR: f64 = 74.0;
/// How hard a shutting lid kicks the case down.
const BUMP: f64 = 0.4;
/// Integration step, seconds.
const STEP: f64 = 0.004;

#[derive(Clone, Copy, Debug)]
struct Spring {
    x: f64,
    v: f64,
    t: f64,
    k: f64,
    c: f64,
}

impl Spring {
    fn new(x: f64, k: f64, c: f64) -> Spring {
        Spring { x, v: 0.0, t: x, k, c }
    }

    fn step(&mut self, dt: f64) {
        self.v += (-self.k * (self.x - self.t) - self.c * self.v) * dt;
        self.x += self.v * dt;
    }

    fn snap(&mut self, t: f64) {
        self.t = t;
        self.x = t;
        self.v = 0.0;
    }
}

/// What a state wants: the lid open, each bud out.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Want {
    lid: bool,
    left: bool,
    right: bool,
}

fn want(pods: Pods) -> Want {
    match pods {
        Pods::Closed => Want { lid: false, left: false, right: false },
        Pods::Open => Want { lid: true, left: false, right: false },
        Pods::OneOut { left: true } => Want { lid: true, left: true, right: false },
        Pods::OneOut { left: false } => Want { lid: true, left: false, right: true },
        Pods::Out => Want { lid: false, left: true, right: true },
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Led {
    Green,
    Amber,
    Off,
}

/// One frame's pose.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pose {
    /// The lid's angle, 0 (shut) to [`OPEN`].
    pub lid: f64,
    /// The case's drop (the bump when the lid shuts), header px.
    pub drop: f64,
    /// The case's opacity.
    pub case: f64,
    pub led: Led,
    /// Each bud's way out of the case (0 seated, 1 out) and its bob (header px, up is negative).
    pub left: (f64, f64),
    pub right: (f64, f64),
}

#[derive(Clone, Debug)]
pub struct Motion {
    pods: Pods,
    lid: Spring,
    left: Spring,
    right: Spring,
    case: Spring,
    bump: Spring,
    /// ms since the motion started, and when the current state began.
    t: f64,
    enter: f64,
}

impl Motion {
    /// At rest in `pods` (the menu just opened).
    pub fn rest(pods: Pods) -> Motion {
        let mut m = Motion {
            pods,
            lid: Spring::new(0.0, 105.0, 13.3),
            left: Spring::new(0.0, 27.0, 9.6),
            right: Spring::new(0.0, 27.0, 9.6),
            case: Spring::new(1.0, 40.0, 11.6),
            bump: Spring::new(0.0, 420.0, 20.0),
            t: 0.0,
            enter: 0.0,
        };
        let w = want(pods);
        m.lid.snap(if w.lid { OPEN } else { 0.0 });
        m.left.snap(if w.left { 1.0 } else { 0.0 });
        m.right.snap(if w.right { 1.0 } else { 0.0 });
        m.case.snap(if pods == Pods::Out { FADED } else { 1.0 });
        m
    }

    pub fn pods(&self) -> Pods {
        self.pods
    }

    /// The buds moved: animate there.
    pub fn go(&mut self, pods: Pods) {
        if pods != self.pods {
            self.pods = pods;
            self.enter = self.t;
        }
    }

    /// Advances by `ms`.
    pub fn advance(&mut self, ms: f64) {
        let mut left = max(0.0, min(ms, 1000.0)) / 1000.0;
        while left > 1e-9 {
            let dt = min(STEP, left);
            self.step(dt);
            left -= dt;
        }
    }

    fn step(&mut self, dt: f64) {
        self.t += dt * 1000.0;
        let w = want(self.pods);
        let lid = self.lid.x;
        let both_out = self.left.x > 0.5 && self.right.x > 0.5;
        self.case.t = if self.pods == Pods::Out && both_out { FADED } else { 1.0 };
        let away = self.case.x < 0.85;
        let mut needs_open = false;
        let (lx, rx) = (self.left.x, self.right.x);
        for left in [true, false] {
            let (des, u) = if left { (w.left, &mut self.left) } else { (w.right, &mut self.right) };
            let cleared = u.x > 0.5;
            let des = if des { 1.0 } else { 0.0 };
            if (des == 1.0 && !cleared) || (des == 0.0 && u.x > 0.02) {
                needs_open = true;
            }
            let mut target = des;
            if lid < LID_CLEAR {
                // A bud waits for the lid.
                if des == 1.0 && !cleared {
                    target = 0.0;
                }
                if des == 0.0 && cleared {
                    target = 1.0;
                }
            }
            if des == 0.0 && cleared && away {
                target = 1.0;
            }
            // The right one follows the left out, and the left one follows the right back.
            if !left && des == 1.0 && w.left && lx < 0.12 && u.x < 0.02 {
                target = 0.0;
            }
            if left && des == 0.0 && !w.right && rx > 0.88 && u.x > 0.98 {
                target = 1.0;
            }
            u.t = target;
            u.step(dt);
            if u.x < 0.0 && u.v < 0.0 {
                // Landed in its well.
                u.x = 0.0;
                u.v = 0.0;
            }
        }
        self.lid.t = if w.lid || needs_open { OPEN } else { 0.0 };
        self.lid.step(dt);
        if self.lid.x < 0.0 {
            // The lid snaps shut, and the case bumps.
            if self.lid.v < -60.0 {
                self.bump.v += min(60.0, -self.lid.v * BUMP);
            }
            self.lid.x = 0.0;
            self.lid.v = 0.0;
        }
        if self.lid.x > 110.0 {
            self.lid.x = 110.0;
            self.lid.v = 0.0;
        }
        self.case.step(dt);
        self.bump.step(dt);
    }

    fn led(&self) -> Led {
        if self.pods == Pods::Out {
            return if self.lid.x > 14.0 { Led::Green } else { Led::Off };
        }
        if self.pods != Pods::Closed || self.lid.x > 14.0 {
            return Led::Green;
        }
        let e = self.t - self.enter;
        if e > BLINK_MS || (crate::math::floor(e / BLINK_STEP_MS) as i64) & 1 == 1 { Led::Off } else { Led::Amber }
    }

    pub fn pose(&self) -> Pose {
        let bud = |u: &Spring, phase: f64| {
            let u = max(0.0, min(u.x, 1.02));
            let bob = if u > 0.97 { -(0.5 - 0.5 * cos(self.t / BOB_MS * 2.0 * PI + phase)) } else { 0.0 };
            (u, bob)
        };
        Pose {
            lid: self.lid.x,
            drop: self.bump.x,
            case: max(0.0, min(self.case.x, 1.0)),
            led: self.led(),
            left: bud(&self.left, 0.0),
            right: bud(&self.right, PI),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settles_on_each_state() {
        let mut m = Motion::rest(Pods::Closed);
        for pods in [Pods::Open, Pods::OneOut { left: true }, Pods::Out, Pods::Open, Pods::Closed] {
            m.go(pods);
            for _ in 0..60 {
                m.advance(94.8);
            }
            let (p, r) = (m.pose(), Motion::rest(pods).pose());
            assert!((p.lid - r.lid).abs() < 0.5, "{pods:?} lid {}", p.lid);
            assert!((p.left.0 - r.left.0).abs() < 0.01 && (p.right.0 - r.right.0).abs() < 0.01, "{pods:?}");
            assert!((p.case - r.case).abs() < 0.01 && p.drop.abs() < 0.05, "{pods:?}");
        }
    }

    #[test]
    fn the_lid_bumps_when_it_shuts_and_the_light_blinks() {
        let mut m = Motion::rest(Pods::Open);
        m.go(Pods::Closed);
        let mut bumped = false;
        let mut amber = false;
        for _ in 0..30 {
            m.advance(50.0);
            bumped |= m.pose().drop > 0.5;
            amber |= m.pose().led == Led::Amber;
        }
        assert!(bumped && amber);
        m.advance(1000.0);
        for _ in 0..6 {
            m.advance(1000.0);
        }
        assert_eq!(m.pose().led, Led::Off);
    }
}
