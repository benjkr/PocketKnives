"""Generates the tray glyphs: assets/airpods-pro-{fill,line}.svg (the "AirPods Pro" icon) and
assets/airpods-{fill,line}.svg (the "AirPods" icon).

    python tools/airpods_glyph.py            (needs: pip install shapely svgpathtools)

AirPods: MingCute's "airpods" fill and line icons (path data in airpods_shapes.json), drawn as
published, with every curve and arc flattened into straight segments (FLAT_STEP long at most) because
the renderer takes only M/L/Z. The line glyph is dimmed to CLASSIC_LINE_OPACITY, as the app first
drew it, and carries a data-tracer orbit around the left head (inside the outline's stroke).

AirPods Pro:

The earbud's shapes (ear tip, tip opening, body, vent, stem) come from airpods_pro_shapes.json: the
regions of one bud of an AirPods Pro line drawing, traced and filled (see "_source" in that file).
This script mirrors the bud into a pair and lays it out for the 16 px tray icon at 100 % scaling,
the size that matters most: a gap between the heads and the stem on two whole pixels. The pair fills
the icon's width; HEAD_TILT and STEM_LENGTH can reshape a bud, but both stay at "as drawn" (0, 1.0).

The app draws the 24-unit box at IconRenderer.Zoom = 1.125, so at 16 px a 24-unit coordinate u
lands on pixel p = 0.75 u - 1. The geometry is built in that 16 px space and written as u = (p + 1) * 4/3.

Fill (connected): body + stem solid, ear tip at TIP_OPACITY, tip opening and vent knocked out, a
gap between tip and body and between head and stem.
Line (not connected): the reverse. The solid parts go hollow (one thin outline of body + stem, stem
joined to the head, with the vent drawn in, at LINE_OPACITY) and the ear tip turns solid at full brightness. The <svg> carries
data-tracer="cx cy rx ry", the orbit the connecting animation's bright spot runs around the left head.
"""
import json, math, os
from svgpathtools import Line, parse_path
from shapely.geometry import Polygon, MultiPolygon
from shapely.geometry.polygon import orient
from shapely.ops import unary_union
from shapely import affinity

Q = 48
TIP_OPACITY = 0.55          # ear tip, connected
LINE_OPACITY = 0.85         # hollow body outline, not connected (thin, so fairly bright)
OUTLINE = 0.7                # outline width, 16 px space (0.45 was too faint at 16 px)
CENTRE_GAP = 1.0             # between the two heads, 16 px space: the smallest that keeps two clear buds
STEM_WIDTH = 2.0             # whole pixels
HEAD_TILT = 0                # degrees, about the body's centre (0 = as drawn)
STEM_LENGTH = 1.0            # stem length factor
MAX_HEIGHT = 14.8            # px; the layout is width-limited unless this is reached
TIP_GAP, STEM_GAP = 0.55, 0.75
CLASSIC_LINE_OPACITY = 0.45  # AirPods, not connected: the outline is a full unit thick
CLASSIC_STROKE = 1.0         # its width, glyph units (the orbit runs along its middle)
FLAT_STEP = 0.3              # longest straight segment a flattened curve becomes, glyph units

HERE = os.path.dirname(os.path.abspath(__file__))


def load_shapes():
    data = json.load(open(os.path.join(HERE, "airpods_pro_shapes.json"), encoding="utf-8"))
    def geom(polys):
        return unary_union([Polygon(rings[0], rings[1:]) for rings in polys]).buffer(0)
    return {k: geom(v) for k, v in data.items() if not k.startswith("_")}


def build():
    R = load_shapes()
    pivot = R["body"].centroid
    for n in ("tip", "noz", "body", "vent"):
        R[n] = affinity.rotate(R[n], HEAD_TILT, origin=pivot)
    R["stem"] = affinity.scale(R["stem"], 1, STEM_LENGTH, origin=(0, R["stem"].bounds[1]))
    x0, y0, x1, y1 = unary_union([R["tip"], R["body"], R["stem"]]).bounds
    k = min((8 - CENTRE_GAP / 2) / (x1 - x0),            # outer edge on the icon edge, inner edge at the gap
            MAX_HEIGHT / (y1 - y0))
    to_px = lambda g: affinity.affine_transform(g, [k, 0, 0, k, -x0 * k, 8 - (y0 + y1) / 2 * k])
    tip, opening, body_f, vent, stem_f = (to_px(R[n]) for n in ("tip", "noz", "body", "vent", "stem"))

    # stem: same centre, exactly STEM_WIDTH wide with its edges and bottom on whole pixels
    sx0, _, sx1, sy1 = stem_f.bounds
    nx0 = round((sx0 + sx1) / 2 - STEM_WIDTH / 2)
    f = STEM_WIDTH / (sx1 - sx0)
    stem_f = affinity.affine_transform(stem_f, [f, 0, 0, 1, nx0 - sx0 * f, round(sy1) - sy1])

    body = body_f.difference(tip.buffer(TIP_GAP, quad_segs=Q))
    stem = stem_f.difference(unary_union([body_f, tip]).buffer(STEM_GAP, quad_segs=Q))
    solid = unary_union([body.difference(vent), stem])
    ear_tip = tip.difference(opening)
    shell = unary_union([body, stem_f.difference(tip.buffer(TIP_GAP, quad_segs=Q))]).buffer(0)
    outline = unary_union([shell.difference(shell.buffer(-OUTLINE, quad_segs=Q)), vent])  # vent drawn, not cut

    hx0, hy0, hx1, hy1 = unary_union([body_f, tip]).bounds
    tracer = ((hx0 + hx1) / 2, (hy0 + hy1) / 2, (hx1 - hx0 - OUTLINE) / 2, (hy1 - hy0 - OUTLINE) / 2)
    return solid, ear_tip, outline, tracer


def pair(g):
    return unary_union([g, affinity.scale(g, xfact=-1, yfact=1, origin=(8, 0))])


def units(v):
    return (v + 1) * 4 / 3


def path_data(g):
    polys = [g] if isinstance(g, Polygon) else list(g.geoms) if isinstance(g, MultiPolygon) else []
    out = []
    num = lambda v: f"{units(v):.3f}".rstrip("0").rstrip(".")
    for p in polys:
        p = orient(p.simplify(0.003), sign=1.0)            # holes wind opposite: nonzero-safe
        for ring in (p.exterior, *p.interiors):
            pts = list(ring.coords)[:-1]
            out.append("M" + "L".join(f"{num(x)} {num(y)}" for x, y in pts) + "Z")
    return "".join(out)


def flat(d):
    """SVG path data (any commands) as straight-line path data, each subpath kept with its own
    winding so the nonzero fill matches the original."""
    num = lambda v: f"{v:.3f}".rstrip("0").rstrip(".")
    out = []
    for sub in parse_path(d).continuous_subpaths():
        pts = [sub.start]
        for seg in sub:
            n = 1 if isinstance(seg, Line) else max(2, math.ceil(seg.length() / FLAT_STEP))
            pts += [seg.point(k / n) for k in range(1, n + 1)]
        if abs(pts[-1] - pts[0]) < 0.02:
            pts.pop()                                        # Z closes it
        out.append("M" + "L".join(f"{num(p.real)} {num(p.imag)}" for p in pts) + "Z")
    return "".join(out)


def classic_tracer(d):
    """The orbit around the left head: the circle the left bud's top is drawn on (as wide as the
    bud), inset to the middle of the outline's stroke."""
    x0, x1, y0 = math.inf, -math.inf, math.inf
    for sub in parse_path(d).continuous_subpaths():
        bx0, bx1, by0, _ = sub.bbox()
        if bx1 <= 12:
            x0, x1, y0 = min(x0, bx0), max(x1, bx1), min(y0, by0)
    r = (x1 - x0) / 2
    return (x0 + x1) / 2, y0 + r, r - CLASSIC_STROKE / 2, r - CLASSIC_STROKE / 2


def main():
    root = os.path.join(HERE, "..", "assets")
    head = '<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"'
    comment = lambda state: f'<!-- QuickTooth tray glyph, {state}. Generated by tools/airpods_glyph.py. -->\n'
    tracer = lambda t: " ".join(f"{v:.3f}" for v in t)

    solid, ear_tip, outline, (cx, cy, rx, ry) = build()
    fill = (f'{head}>\n{comment("connected")}'
            f'<path fill="currentColor" d="{path_data(pair(solid))}"/>\n'
            f'<path fill="currentColor" fill-opacity="{TIP_OPACITY:g}" d="{path_data(pair(ear_tip))}"/>\n</svg>\n')
    tr = tracer((units(cx), units(cy), rx * 4 / 3, ry * 4 / 3))
    line = (f'{head} data-tracer="{tr}">\n{comment("not connected")}'
            f'<path fill="currentColor" fill-opacity="{LINE_OPACITY:g}" d="{path_data(pair(outline))}"/>\n'
            f'<path fill="currentColor" d="{path_data(pair(ear_tip))}"/>\n</svg>\n')

    src = json.load(open(os.path.join(HERE, "airpods_shapes.json"), encoding="utf-8"))
    classic_fill = f'{head}>\n{comment("connected")}<path fill="currentColor" d="{flat(src["fill"])}"/>\n</svg>\n'
    classic_tr = tracer(classic_tracer(src["line"]))
    classic_line = (f'{head} data-tracer="{classic_tr}">\n{comment("not connected")}'
                    f'<path fill="currentColor" fill-opacity="{CLASSIC_LINE_OPACITY:g}" d="{flat(src["line"])}"/>\n</svg>\n')

    for name, text in (("airpods-pro-fill.svg", fill), ("airpods-pro-line.svg", line),
                       ("airpods-fill.svg", classic_fill), ("airpods-line.svg", classic_line)):
        with open(os.path.join(root, name), "w", encoding="utf-8", newline="\n") as fh:
            fh.write(text)
        print(name, len(text), "bytes")
    print("tracer", tr, "/", classic_tr)


if __name__ == "__main__":
    main()
