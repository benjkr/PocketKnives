//! Async without a runtime: the controller's waits (`refresh`, `delay`, the menu, ...) are Rust
//! futures, driven by a tiny single-threaded executor whose wake-ups each host schedules.
//!
//! The semantics are C#'s `async`/`await`, which the logs and timelines were recorded with:
//! - [`spawn`] polls the task at once, so it runs synchronously up to its first wait that isn't
//!   already complete (an async method called with `_ = M()`).
//! - Awaiting an [`Op`] that is already complete continues without yielding.
//! - A woken task is handed to the host's [`Schedule`]: the exe posts a window message per
//!   wake-up (its `SynchronizationContext` did the same); the browser runs it at once, in order,
//!   never nested ([`Inline`], what `InlineContext` did).
//!
//! Everything here is single-threaded (`Rc`): an [`Op`]'s [`Done`] is completed on the thread
//! that owns the tasks. A host whose work finishes on another thread posts the result to its UI
//! thread first.
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// What a host operation gives back: its value, or the error text that the C# version's
/// exception carried (logged by the controller as `ERROR <context>: <text>`).
pub type OpResult<T> = Result<T, String>;

// ---------------------------------------------------------------------------- Op / Done

struct Slot<T> {
    value: RefCell<Option<OpResult<T>>>,
    waker: RefCell<Option<Waker>>,
}

/// A host operation's result, awaited by the controller.
pub struct Op<T> {
    slot: Rc<Slot<T>>,
}

/// The other end of an [`Op`]: the host completes it once.
pub struct Done<T> {
    slot: Rc<Slot<T>>,
}

/// A new pending operation.
pub fn op<T>() -> (Op<T>, Done<T>) {
    let slot = Rc::new(Slot { value: RefCell::new(None), waker: RefCell::new(None) });
    (Op { slot: slot.clone() }, Done { slot })
}

impl<T> Op<T> {
    /// An operation that has already finished (awaiting it doesn't yield).
    pub fn ready(value: OpResult<T>) -> Op<T> {
        let (op, done) = op();
        *done.slot.value.borrow_mut() = Some(value);
        op
    }

    pub fn ok(value: T) -> Op<T> {
        Op::ready(Ok(value))
    }
}

impl<T> Done<T> {
    /// Stores the result and wakes the task awaiting it (through its host's [`Schedule`]).
    pub fn complete(self, value: OpResult<T>) {
        *self.slot.value.borrow_mut() = Some(value);
        let waker = self.slot.waker.borrow_mut().take();
        if let Some(w) = waker {
            w.wake();
        }
    }

    pub fn ok(self, value: T) {
        self.complete(Ok(value));
    }
}

impl<T> Future for Op<T> {
    type Output = OpResult<T>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<OpResult<T>> {
        if let Some(v) = self.slot.value.borrow_mut().take() {
            return Poll::Ready(v);
        }
        *self.slot.waker.borrow_mut() = Some(cx.waker().clone());
        Poll::Pending
    }
}

// ---------------------------------------------------------------------------- tasks

/// Decides when a woken task runs. The host calls [`Task::run`] later (the exe, from a posted
/// window message) or at once ([`Inline`]).
pub trait Schedule {
    fn schedule(&self, task: Task);
}

type BoxFuture = Pin<Box<dyn Future<Output = ()>>>;

struct TaskInner {
    future: RefCell<Option<BoxFuture>>,
    /// Being polled right now (a wake-up during the poll re-schedules it afterwards).
    running: Cell<bool>,
    woken_while_running: Cell<bool>,
    /// Already handed to the scheduler and not run yet.
    scheduled: Cell<bool>,
    done: Cell<bool>,
    schedule: Rc<dyn Schedule>,
}

/// A spawned future. Cloning gives another handle to the same task.
#[derive(Clone)]
pub struct Task(Rc<TaskInner>);

/// Starts `future` and runs it at once up to its first pending wait. Later wake-ups go to
/// `schedule`.
pub fn spawn(schedule: &Rc<dyn Schedule>, future: impl Future<Output = ()> + 'static) {
    let task = Task(Rc::new(TaskInner {
        future: RefCell::new(Some(Box::pin(future))),
        running: Cell::new(false),
        woken_while_running: Cell::new(false),
        scheduled: Cell::new(false),
        done: Cell::new(false),
        schedule: schedule.clone(),
    }));
    task.run();
}

impl Task {
    /// Polls the task once (a scheduled wake-up). Running a finished task does nothing.
    pub fn run(self) {
        let inner = &self.0;
        inner.scheduled.set(false);
        if inner.done.get() {
            return;
        }
        if inner.running.get() {
            // Only possible if a poll ran the scheduler's queue; remember and poll again after.
            inner.woken_while_running.set(true);
            return;
        }
        // Take the future out while polling, so code it runs can spawn or wake other tasks.
        let Some(mut future) = inner.future.borrow_mut().take() else { return };
        inner.running.set(true);
        let waker = waker_for(&self.0);
        let mut cx = Context::from_waker(&waker);
        let ready = future.as_mut().poll(&mut cx).is_ready();
        inner.running.set(false);
        if ready {
            inner.done.set(true);
            return;
        }
        *inner.future.borrow_mut() = Some(future);
        if inner.woken_while_running.replace(false) {
            self.wake();
        }
    }

    fn wake(self) {
        let inner = &self.0;
        if inner.done.get() {
            return;
        }
        if inner.running.get() {
            inner.woken_while_running.set(true);
            return;
        }
        if inner.scheduled.replace(true) {
            return;
        }
        let schedule = inner.schedule.clone();
        schedule.schedule(self);
    }
}

// A waker is an `Rc<TaskInner>` in disguise. Sound because tasks and their wakers never leave the
// thread that runs them: `Op`s are completed on that thread too.
fn waker_for(task: &Rc<TaskInner>) -> Waker {
    let raw = RawWaker::new(Rc::into_raw(task.clone()) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |p| {
        let rc = unsafe { Rc::from_raw(p as *const TaskInner) };
        let clone = rc.clone();
        core::mem::forget(rc);
        RawWaker::new(Rc::into_raw(clone) as *const (), &VTABLE)
    },
    |p| Task(unsafe { Rc::from_raw(p as *const TaskInner) }).wake(),
    |p| {
        let rc = unsafe { Rc::from_raw(p as *const TaskInner) };
        Task(rc.clone()).wake();
        core::mem::forget(rc);
    },
    |p| drop(unsafe { Rc::from_raw(p as *const TaskInner) }),
);

// ---------------------------------------------------------------------------- Inline

/// Runs a woken task at once, in order, never nested: a wake-up that arrives while tasks are
/// already running waits its turn in the queue. The browser host uses it.
#[derive(Default)]
pub struct Inline {
    queue: RefCell<VecDeque<Task>>,
    draining: Cell<bool>,
}

impl Schedule for Inline {
    fn schedule(&self, task: Task) {
        self.queue.borrow_mut().push_back(task);
        if self.draining.replace(true) {
            return;
        }
        loop {
            let next = self.queue.borrow_mut().pop_front();
            match next {
                Some(t) => t.run(),
                None => break,
            }
        }
        self.draining.set(false);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use alloc::vec::Vec;

    /// Records the order things happen in.
    fn trace() -> Rc<RefCell<Vec<&'static str>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn runs_synchronously_until_the_first_pending_wait() {
        let sched: Rc<dyn Schedule> = Rc::new(Inline::default());
        let t = trace();
        let (op, done) = op::<i32>();
        let t2 = t.clone();
        spawn(&sched, async move {
            t2.borrow_mut().push("start");
            let v = Op::ok(1).await.unwrap();
            assert_eq!(v, 1);
            t2.borrow_mut().push("after ready");
            let v = op.await.unwrap();
            assert_eq!(v, 7);
            t2.borrow_mut().push("after pending");
        });
        t.borrow_mut().push("spawn returned");
        done.ok(7);
        t.borrow_mut().push("complete returned");
        assert_eq!(*t.borrow(), ["start", "after ready", "spawn returned", "after pending", "complete returned"]);
    }

    #[test]
    fn inline_wakeups_run_in_order_and_never_nest() {
        let sched: Rc<dyn Schedule> = Rc::new(Inline::default());
        let t = trace();
        let (a, done_a) = op::<()>();
        let (b, done_b) = op::<()>();
        let done_b = Rc::new(RefCell::new(Some(done_b)));
        let (t1, t2, db) = (t.clone(), t.clone(), done_b.clone());
        spawn(&sched, async move {
            a.await.unwrap();
            t1.borrow_mut().push("a resumed");
            // Completing b from inside a running task queues b's task behind this one.
            db.borrow_mut().take().unwrap().ok(());
            t1.borrow_mut().push("a finished");
        });
        spawn(&sched, async move {
            b.await.unwrap();
            t2.borrow_mut().push("b resumed");
        });
        done_a.ok(());
        assert_eq!(*t.borrow(), ["a resumed", "a finished", "b resumed"]);
    }

    #[test]
    fn spawn_inside_a_task_runs_at_once() {
        let sched: Rc<dyn Schedule> = Rc::new(Inline::default());
        let t = trace();
        let (op, done) = op::<()>();
        let (t1, s2) = (t.clone(), sched.clone());
        spawn(&sched, async move {
            op.await.unwrap();
            let t3 = t1.clone();
            spawn(&s2, async move { t3.borrow_mut().push("inner") });
            t1.borrow_mut().push("outer");
        });
        done.ok(());
        assert_eq!(*t.borrow(), ["inner", "outer"]);
    }

    /// A host that runs wake-ups later, like the exe's posted messages.
    #[derive(Default)]
    struct Deferred(RefCell<VecDeque<Task>>);
    impl Schedule for Deferred {
        fn schedule(&self, task: Task) {
            self.0.borrow_mut().push_back(task);
        }
    }

    #[test]
    fn deferred_wakeups_wait_for_the_host() {
        let host = Rc::new(Deferred::default());
        let sched: Rc<dyn Schedule> = host.clone();
        let t = trace();
        let (op, done) = op::<()>();
        let t1 = t.clone();
        spawn(&sched, async move {
            op.await.unwrap();
            t1.borrow_mut().push("resumed");
        });
        done.ok(());
        assert!(t.borrow().is_empty());
        let task = host.0.borrow_mut().pop_front().unwrap();
        task.run();
        assert_eq!(*t.borrow(), ["resumed"]);
        assert!(host.0.borrow().is_empty());
    }
}
