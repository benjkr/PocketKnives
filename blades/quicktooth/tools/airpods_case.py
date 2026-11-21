"""Generates tools/airpods_case_closed.svg and tools/airpods_case_open.svg: the charging case
drawn for QuickTooth to go with the bud drawing (tools/header_art.py then shades both).

    python tools/airpods_case.py            (needs: pip install shapely svgpathtools)

The case is a small 3D model: a stack of horizontal slices (a rounded-rectangle plan, inset where
the top and bottom edges round over), seen from the front by a camera pitched down, orthographic.
The projected slices give the silhouette; the seam, the rim, the wells and the lid's rim are
centre lines on that model. Every centre line is widened to LW and a layer's lines are merged, so
each layer is one filled outline with even-odd holes, like the bud drawing.
"""
import math
import os
import re
import sys
from shapely import affinity
from shapely.geometry import LineString, MultiPolygon, Point, Polygon, box
from shapely.ops import unary_union
from svgpathtools import Line, parse_path

HERE = os.path.dirname(os.path.abspath(__file__))

# ---- the model, in drawing units (the case is W wide; 1 unit = 0.56 mm)
W, D, H = 108.0, 38.0, 74.0     # width, depth, height
RC = D / 2 * 0.94               # the plan's corner radius: almost a stadium
RV = 15.0                       # how the top and bottom edges round over
SEAM = H * 0.66                 # the lid seam's height: the lid is the top third
PITCH = math.radians(21)        # the camera looks down by this much
OPEN = math.radians(102)        # how far the lid swings back
LW = 1.1                        # line weight: the buds' 1.0 at the case's slightly smaller scale
WELL_X, WELL_W, WELL_D, WELL_R = 25.0, 42.0, 22.0, 8.0   # each well's centre offset and plan size
LIGHT_R = 1.45                  # the status light: a ring, to its centre line
CSS = '''  body { margin: 0; font: 12px/1.4 "Segoe UI", sans-serif; }
  section { padding: 16px; }
  section.dark { background: #2C2C2C; color: #F2F2F2; }
  section.light { background: #F9F9F9; color: #282828; }
  h2 { font-size: 13px; margin: 0 0 8px; }
  figure { margin: 0 0 14px; }
  figcaption { opacity: .7; margin-bottom: 4px; }
  .pic { display: block; outline: 1px dashed rgba(128,128,128,.35); margin-bottom: 6px; }
  .row { display: flex; gap: 16px; align-items: flex-start; }
'''
HINGE_W, HINGE_H = 26.0, 5.5
EDGE = 4.6                      # the closed lid's top edge line: this far inside the outline
CX, BOTTOM = 64.0, 117.0        # where the case sits in the 128 box (its lowest point)

# ---- the seated buds, relative to the bud drawing
SEAT_SCALE = 0.72
SEAT_CUT = 54.0                 # the bud drawing's y (left bud) that meets the well's front edge
HALO = 0.9                      # case lines stop this far from a seated bud

SP, CP = math.sin(PITCH), math.cos(PITCH)


def plan(inset=0.0):
    core = box(-W / 2 + RC, -D / 2 + RC, W / 2 - RC, D / 2 - RC)
    return core.buffer(RC - inset, 48)


def inset_at(z):
    """How far the plan shrinks at height z (the rounded top and bottom edges)."""
    t = RV - z if z < RV else (z - (H - RV) if z > H - RV else 0.0)
    return RV - math.sqrt(max(0.0, RV * RV - t * t))


def body_xf(z):
    """Plan (x, y) at height z -> the page (before the final shift)."""
    return [1, 0, 0, -SP, 0, -z * CP]


S_TP = math.sin(OPEN - PITCH)


def lid_xf(z):
    """Plan (x, y) at height z of the lid, swung back by OPEN about the back edge at the seam."""
    yh, dz = D / 2, z - SEAM
    # y' = yh + (y - yh) cos + dz sin ; z' = SEAM - (y - yh) sin + dz cos ; page = -z' CP - y' SP
    c, s = math.cos(OPEN), math.sin(OPEN)
    const = -CP * (SEAM + yh * s + dz * c) - SP * (yh - yh * c + dz * s)
    return [1, 0, 0, S_TP, 0, const]


def zs(z0, z1, n=140):
    return [z0 + (z1 - z0) * k / n for k in range(n + 1)]


def solid(z0, z1, xf):
    parts = [affinity.affine_transform(plan(inset_at(z)), xf(z)) for z in zs(z0, z1)]
    return unary_union(parts).buffer(0.25).buffer(-0.25)


def half(poly, front):
    """The front (y <= 0) or back half of a plan outline, as points from left to right."""
    pts = [p for p in poly.exterior.coords[:-1] if (p[1] <= 1e-9 if front else p[1] >= -1e-9)]
    pts.sort(key=lambda p: math.atan2(p[1], p[0]) if front else -math.atan2(p[1], p[0]))
    return pts


def tf(pts, m):
    return [(m[0] * x + m[1] * y + m[4], m[2] * x + m[3] * y + m[5]) for x, y in pts]


def stroke(pts, w=LW):
    return LineString(pts).buffer(w / 2, 12)


def ring(poly, w=LW):
    return poly.exterior.buffer(w / 2, 12)


def well(side):
    core = box(side * WELL_X - WELL_W / 2 + WELL_R, -WELL_D / 2 + WELL_R, side * WELL_X + WELL_W / 2 - WELL_R, WELL_D / 2 - WELL_R)
    return core.buffer(WELL_R, 24)


def well_half(side, front):
    w = well(side)
    pts = [(x - side * WELL_X, y) for x, y in w.exterior.coords[:-1]]
    pts = [p for p in pts if (p[1] <= 1e-9 if front else p[1] >= -1e-9)]
    pts.sort(key=lambda p: math.atan2(p[1], p[0]) if front else -math.atan2(p[1], p[0]))
    return [(x + side * WELL_X, y) for x, y in pts]


# ---- the bud drawing

def flat(d, step=0.3):
    out = []
    for sub in parse_path(d).continuous_subpaths():
        pts = []
        for seg in sub:
            n = 1 if isinstance(seg, Line) else max(2, int(seg.length() / step) + 1)
            for k in range(n):
                p = seg.point(k / n)
                pts.append((p.real, p.imag))
        if len(pts) >= 3:
            out.append(pts)
    return out


def shape(d):
    g = Polygon()
    for r in flat(d):
        g = g.symmetric_difference(Polygon(r).buffer(0))
    return g


def filled(g):
    """A shape with its holes filled."""
    polys = g.geoms if isinstance(g, MultiPolygon) else [g]
    return unary_union([Polygon(p.exterior) for p in polys])


# ---- output

def path_d(g):
    polys = [g] if isinstance(g, Polygon) else [p for p in getattr(g, "geoms", []) if isinstance(p, Polygon)]
    parts = []
    for p in polys:
        if p.area < 0.02:
            continue
        for r in [p.exterior, *p.interiors]:
            c = list(r.simplify(0.02).coords)[:-1]
            parts.append("M" + "L".join(f"{x:.2f} {y:.2f}" for x, y in c) + "Z")
    return "".join(parts)


def path(g, pid=None):
    d = path_d(g)
    if not d:
        return ""
    return f'<path{" id=" + chr(34) + pid + chr(34) if pid else ""} fill="#000" fill-rule="evenodd" d="{d}"/>'


def main():
    src = open(os.path.join(HERE, "airpods_pro_drawing.svg"), encoding="utf-8").read()
    ds = re.findall(r'<path[^>]*\sd="([^"]+)"', src)
    assert len(ds) == 16
    buds = {}
    for side, part in (("left", ds[:8]), ("right", ds[8:])):
        ink = unary_union([shape(d) for d in part]).buffer(0)
        buds[side] = {"d": part, "ink": ink, "sil": filled(filled(ink.buffer(0.2))), "b": ink.bounds}

    closed_sil = solid(0, H, body_xf)
    shift_y = BOTTOM - closed_sil.bounds[3]

    def place(g):
        return affinity.translate(g, CX, shift_y)

    def body_pt(x, y, z):
        return (CX + x, shift_y - z * CP - y * SP)

    # ---------------- closed
    seam = tf(half(plan(), True), body_xf(SEAM))
    light = Point(body_pt(0, -D / 2, SEAM - 9.0)).buffer(LIGHT_R, 16).exterior.buffer(LW / 2, 12)
    # The lid's rounded top edge: a second line inside the outline, across the top and into the
    # corners, like the second contour along the top of each bud.
    inner = place(closed_sil).buffer(-EDGE, 32).exterior
    top = place(closed_sil).bounds[1]
    edge = inner.intersection(box(-10, -10, 140, top + EDGE + RV * 0.55)).buffer(LW / 2, 12)
    closed = unary_union([ring(place(closed_sil)), place(stroke(seam)), light, edge])

    # ---------------- open
    body_sil = place(solid(0, SEAM, body_xf))
    y_side = body_pt(0, 0, SEAM)[1]                       # the rim's leftmost and rightmost points
    outline = body_sil.exterior
    lower = box(-10, y_side, 140, 140)
    front_u = outline.intersection(lower).buffer(LW / 2, 12)
    back_rim = outline.difference(lower).buffer(LW / 2, 12)
    front_rim = place(stroke(tf(half(plan(), True), body_xf(SEAM))))
    wells_front = [place(stroke(tf(well_half(s, True), body_xf(SEAM)))) for s in (-1, 1)]
    wells_back = [place(stroke(tf(well_half(s, False), body_xf(SEAM)))) for s in (-1, 1)]
    cut_y = body_pt(0, -WELL_D / 2, SEAM)[1]              # the wells' front edge on the page

    lid_sil = place(solid(SEAM, H, lid_xf))
    lid_rim = place(affinity.affine_transform(plan(), lid_xf(SEAM)))
    m = lid_xf(SEAM)
    hy0 = shift_y + m[3] * (D / 2) + m[5]                 # the hinge line on the page
    hy1 = hy0 - HINGE_H
    r = 1.6
    hinge = stroke([(CX - HINGE_W / 2, hy0), (CX - HINGE_W / 2, hy1 + r)] +
                   [(CX - HINGE_W / 2 + r - r * math.cos(a), hy1 + r - r * math.sin(a)) for a in [math.pi / 2 * k / 6 for k in range(7)]] +
                   [(CX + HINGE_W / 2 - r + r * math.sin(a), hy1 + r - r * math.cos(a)) for a in [math.pi / 2 * k / 6 for k in range(7)]] +
                   [(CX + HINGE_W / 2, hy0)])
    open_light = Point(body_pt(0, -D / 2, SEAM - 9.0)).buffer(LIGHT_R, 16).exterior.buffer(LW / 2, 12)

    back = unary_union([ring(lid_sil), ring(lid_rim), hinge, back_rim, *wells_back])
    front = unary_union([front_u, front_rim, *wells_front, open_light])

    # ---------------- the seats
    seats = {}
    for side, sgn in (("left", -1), ("right", 1)):
        b = buds[side]["b"]
        bw, bh = b[2] - b[0], b[3] - b[1]
        cut_v = SEAT_CUT + (b[1] - buds["left"]["b"][1])    # the same place on either bud
        ax = CX + sgn * WELL_X - SEAT_SCALE * bw / 2
        ay = cut_y - SEAT_SCALE * (cut_v - b[1])
        sil = affinity.affine_transform(buds[side]["sil"], [SEAT_SCALE, 0, 0, SEAT_SCALE, ax - SEAT_SCALE * b[0], ay - SEAT_SCALE * b[1]])
        sil = sil.intersection(box(-10, -10, 140, cut_y)).buffer(HALO)
        sil = sil.intersection(box(-10, -10, 140, cut_y - LW / 2))   # the well's own front edge stays whole
        # A seated bud fills its well: the well's back edge goes with it.
        sil = unary_union([sil, wells_back[0 if sgn < 0 else 1].buffer(0.05)])
        seats[side] = {"ax": ax, "ay": ay, "w": bw * SEAT_SCALE, "h": bh * SEAT_SCALE, "sil": sil, "b": b}
    both = unary_union([seats["left"]["sil"], seats["right"]["sil"]])

    def split(g, name):
        return (path(g.difference(both), f"{name}-always"),
                path(g.intersection(seats["left"]["sil"]), f"{name}-left-empty"),
                path(g.intersection(seats["right"]["sil"]), f"{name}-right-empty"))

    back_parts, front_parts = split(back, "back"), split(front, "front")

    def bounds(g):
        return tuple(round(v, 2) for v in g.bounds)

    cb, ob = bounds(closed), bounds(unary_union([back, front]))
    note_closed = f"""<!--
  AirPods Pro charging case, closed. Line art to go with tools/airpods_pro_drawing.svg: filled
  outlines {LW} units thick, even-odd. Generated by gen_case.py; don't edit.
  Ink bounds: x {cb[0]}..{cb[2]}, y {cb[1]}..{cb[3]}. The body below the seam is the same shape, in
  the same place, as in airpods_case_open.svg, so the two swap without moving.
-->"""
    sl, sr = seats["left"], seats["right"]
    note_open = f"""<!--
  AirPods Pro charging case, open. Line art to go with tools/airpods_pro_drawing.svg: filled
  outlines {LW} units thick, even-odd. Generated by gen_case.py; don't edit.
  Ink bounds: x {ob[0]}..{ob[2]}, y {ob[1]}..{ob[3]}. Same scale and body position as airpods_case_closed.svg.

  Layers. #back: the lid (silhouette, rim, hinge plate), the body's back rim and the wells' back
  edges. #front: the body's front wall, the front rim, the wells' front edges and the status light.
  Each layer is split in three, because the header is one ink layer and nothing erases:
    *-always       drawn in every open state;
    *-left-empty   the lines a seated LEFT bud hides (its well's back edge too): draw only
                   while the left well is empty;
    *-right-empty  the same for the right bud.

  Seats (in this file's units). A seated bud is the bud drawing scaled by {SEAT_SCALE}, placed by the
  top-left of its ink bounds, and cut off below y = {cut_y:.2f} (a horizontal cut: the wells' front
  edge is straight across the bud).
    left bud  (source paths 1-8,  ink bounds {tuple(round(v, 2) for v in sl['b'])}):
        top-left at x = {sl['ax']:.2f}, y = {sl['ay']:.2f}; scale {SEAT_SCALE}; size {sl['w']:.2f} x {sl['h']:.2f}; cut below y = {cut_y:.2f}
    right bud (source paths 9-16, ink bounds {tuple(round(v, 2) for v in sr['b'])}):
        top-left at x = {sr['ax']:.2f}, y = {sr['ay']:.2f}; scale {SEAT_SCALE}; size {sr['w']:.2f} x {sr['h']:.2f}; cut below y = {cut_y:.2f}
  That is: page = top-left + {SEAT_SCALE} * (bud point - bud ink bounds' top-left).
-->"""
    head = '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128" width="512" height="512">'
    closed_svg = f"{head}\n{note_closed}\n{path(closed)}\n</svg>\n"
    open_svg = (f"{head}\n{note_open}\n<g id=\"back\">\n" + "\n".join(p for p in back_parts if p) +
                "\n</g>\n<g id=\"front\">\n" + "\n".join(p for p in front_parts if p) + "\n</g>\n</svg>\n")
    open(os.path.join(HERE, "airpods_case_closed.svg"), "w", encoding="utf-8", newline="\n").write(closed_svg)
    open(os.path.join(HERE, "airpods_case_open.svg"), "w", encoding="utf-8", newline="\n").write(open_svg)
    return
    print("closed", cb, "open", ob, "cut_y", round(cut_y, 2), "hinge", round(hy0, 2))
    for k, s in seats.items():
        print(k, round(s["ax"], 2), round(s["ay"], 2), round(s["w"], 2), round(s["h"], 2))
    print(len(closed_svg), len(open_svg))

    # ---------------- the preview
    K = 60.0 / W                          # header px per case unit: the case is 60 px wide
    PW, PH = 236, 60
    case_x, case_y = PW / 2 - CX * K, 54.0 - BOTTOM * K   # the case's bottom 54 px down the picture
    out_h, side_x, out_top = 45.0, 40.0, 4.0

    def bud_paths(side):
        return "".join(f'<path fill-rule="evenodd" d="{d}"/>' for d in buds[side]["d"])

    def standing(side):
        b = buds[side]["b"]
        s = out_h / (b[3] - b[1])
        cx = side_x if side == "left" else PW - side_x
        x = cx - s * (b[2] - b[0]) / 2
        return f'<g transform="translate({x - s * b[0]:.3f} {out_top - s * b[1]:.3f}) scale({s:.4f})">{bud_paths(side)}</g>'

    def seated(side):
        st = seats[side]
        b = st["b"]
        return (f'<g clip-path="url(#cut)"><g transform="translate({st["ax"] - SEAT_SCALE * b[0]:.3f} {st["ay"] - SEAT_SCALE * b[1]:.3f}) '
                f'scale({SEAT_SCALE})">{bud_paths(side)}</g></g>')

    def case(inner, opacity=1.0):
        return f'<g transform="translate({case_x:.3f} {case_y:.3f}) scale({K:.5f})" opacity="{opacity}">{inner}</g>'

    def strip(p):
        return re.sub(r' fill="#000"', "", p)

    bk, fr = [strip(p) for p in back_parts], [strip(p) for p in front_parts]
    states = [
        ("Closed", case(strip(path(closed)))),
        ("Open, both buds in", case(bk[0] + seated("left") + seated("right") + fr[0])),
        ("Left bud out", case(bk[0] + bk[1] + seated("right") + fr[0] + fr[1]) + standing("left")),
        ("Right bud out", case(bk[0] + bk[2] + seated("left") + fr[0] + fr[2]) + standing("right")),
        ("Both out: case closed and faint", case(strip(path(closed)), 0.4) + standing("left") + standing("right")),
        ("Open, empty (not a header state; shows the whole drawing)", case(bk[0] + bk[1] + bk[2] + fr[0] + fr[1] + fr[2])),
    ]
    defs = f'<defs><clipPath id="cut"><rect x="-10" y="-10" width="150" height="{cut_y + 10:.3f}"/></clipPath></defs>'

    def pic(inner, zoom):
        return (f'<svg class="pic" width="{PW * zoom}" height="{PH * zoom}" viewBox="0 0 {PW} {PH}" fill="currentColor">{defs}{inner}</svg>')

    rows = []
    for theme in ("dark", "light"):
        cells = "".join(f'<figure><figcaption>{name}</figcaption><div class="row">{pic(inner, 1)}{pic(inner, 3)}</div></figure>' for name, inner in states)
        rows.append(f'<section class="{theme}"><h2>{theme.title()} menu: header size, and 3×</h2>{cells}</section>')
    for theme, row in zip(("dark", "light"), rows):
        open(os.path.join(HERE, f"check-{theme}.html"), "w", encoding="utf-8", newline="\n").write("<!doctype html><meta charset=utf-8><style>" + CSS + "</style>" + row)
    html = f"""<!doctype html>
<meta charset="utf-8">
<title>QuickTooth case line art</title>
<style>
{CSS}</style>
<p style="margin:0;padding:8px 16px;background:#444;color:#eee">Each state at the header's size (236 × 60 px picture, case 60 px wide,
standing buds 45 px tall) and at 3×. One ink colour, no fills, seated buds cut at a horizontal line: what the header's renderer can draw.</p>
{''.join(rows)}
"""
    open(os.path.join(HERE, "case-preview.html"), "w", encoding="utf-8", newline="\n").write(html)
    print("preview", len(html))


if __name__ == "__main__":
    main()
