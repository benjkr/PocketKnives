"""Generates assets/header-case-lid-<deg>-back.svg: the open case's back half with the lid part of
the way open, for the menu header's lid animation, from the case's 3D model (tools/airpods_case.py)
and the header's shading (tools/header_art.py). Same 128-unit box and place as
assets/header-case-*.svg; the front half is the same at every angle (header-case-open-front.svg).
Prints each frame's top, hinge line and lid front edge (where seated buds are cut off).

    python tools/header_lid_frames.py            (needs: pip install shapely svgpathtools)
"""
import math, os, sys
TOOLS = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, TOOLS)
import airpods_case as ac
import header_art as ha
from shapely import affinity
from shapely.geometry import Point, box
from shapely.ops import unary_union

OUT = os.path.join(TOOLS, "..", "assets")
ANGLES = [int(v) for v in sys.argv[1:]] or [30, 54, 78]
HW = ac.LW / 2
shift_y = ac.BOTTOM - ac.solid(0, ac.H, ac.body_xf).bounds[3]


def place(g):
    return affinity.translate(g, ac.CX, shift_y)


def body_pt(x, y, z):
    return (ac.CX + x, shift_y - z * ac.CP - y * ac.SP)


def lid_xf(z, a):
    yh, dz = ac.D / 2, z - ac.SEAM
    c, s = math.cos(a), math.sin(a)
    const = -ac.CP * (ac.SEAM + yh * s + dz * c) - ac.SP * (yh - yh * c + dz * s)
    return [1, 0, 0, math.sin(a - ac.PITCH), 0, const]


def build(deg):
    a = math.radians(deg)
    body_sil = place(ac.solid(0, ac.SEAM, ac.body_xf))
    mouth = place(affinity.affine_transform(ac.plan(), ac.body_xf(ac.SEAM)))
    wall = body_sil.difference(mouth).buffer(0)
    lid_sil = place(ac.solid(ac.SEAM, ac.H, lambda z: lid_xf(z, a)))
    m = lid_xf(ac.SEAM, a)
    lid_rim = place(affinity.affine_transform(ac.plan(), m))
    y_side = body_pt(0, 0, ac.SEAM)[1]
    lower = box(-10, y_side, 140, 140)
    front_u = body_sil.exterior.intersection(lower).buffer(HW, 12)
    back_rim = body_sil.exterior.difference(lower).buffer(HW, 12)
    front_rim = place(ac.stroke(ac.tf(ac.half(ac.plan(), True), ac.body_xf(ac.SEAM))))
    wells_f = [place(ac.stroke(ac.tf(ac.well_half(s, True), ac.body_xf(ac.SEAM)))) for s in (-1, 1)]
    wells_b = [place(ac.stroke(ac.tf(ac.well_half(s, False), ac.body_xf(ac.SEAM)))) for s in (-1, 1)]
    led_c = Point(body_pt(0, -ac.D / 2, ac.SEAM - 9.0))
    light = led_c.buffer(ac.LIGHT_R, 16).exterior.buffer(HW, 12)
    hy0 = shift_y + m[3] * (ac.D / 2) + m[5]          # the hinge line on the page
    rim_front = shift_y + m[3] * (-ac.D / 2) + m[5]   # the lid rim's front edge on the page
    extra = []
    if hy0 - rim_front > ac.HINGE_H + 6:
        hy1, r, hwid = hy0 - ac.HINGE_H, 1.6, ac.HINGE_W / 2
        q = [math.pi / 2 * k / 6 for k in range(7)]
        extra.append(ac.stroke([(ac.CX - hwid, hy0), (ac.CX - hwid, hy1 + r)] +
                               [(ac.CX - hwid + r - r * math.cos(t), hy1 + r - r * math.sin(t)) for t in q] +
                               [(ac.CX + hwid - r + r * math.sin(t), hy1 + r - r * math.cos(t)) for t in q] +
                               [(ac.CX + hwid, hy0)]))
    back_lines = unary_union([ac.ring(lid_sil), ac.ring(lid_rim), back_rim, *wells_b, *extra]).buffer(0)
    front_lines = unary_union([front_u, front_rim, *wells_f, light]).buffer(0)
    lines = unary_union([back_lines, front_lines]).buffer(0)
    sil = unary_union([body_sil, lid_sil]).buffer(0)
    wall_in = wall.buffer(-HW)
    face = lid_rim.buffer(-HW)
    crescent = lid_sil.difference(lid_rim.buffer(HW)).buffer(-HW)
    f = math.sin(a - ac.PITCH) / math.sin(ac.OPEN - ac.PITCH)   # how much of the lid's inside shows

    back = ha.Art()
    ha.shadow(back, body_sil, 3.0)
    back.add("shell", sil.difference(wall_in))
    back.material(crescent, "shell", "shell-hi", "shell-lo", None, k=(2.0, 5.0, 2.0))
    back.material(face, "lid", "lid-hi", "lid-lo", None, k=(2.0 * f, 6.0 * f, 2.0))
    if extra:
        hin = box(ac.CX - ac.HINGE_W / 2 + HW, hy0 - ac.HINGE_H + HW, ac.CX + ac.HINGE_W / 2 - HW, hy0 - HW)
        back.add("hinge", hin.intersection(face))
    back.add("mouth", mouth.buffer(-HW))
    back.add("well", mouth.buffer(-HW - 2.2))
    back.lines(back_lines.difference(wall_in.buffer(0.6)), sil)
    front = ha.Art()
    front.material(wall_in, "shell", "shell-hi", "shell-lo", "shell-lo2", k=(2.0, 9.0, 3.5))
    front.add("led", led_c.buffer(ac.LIGHT_R - HW + 1.4))
    front.lines(lines.intersection(wall_in.buffer(1.4)).union(front_lines).buffer(0), sil)
    note = f"tools/airpods_case.py's model, lid at {deg} degrees"
    for half, art in (("back", back),):   # the front half is the same at every angle: assets/header-case-open-front.svg
        open(os.path.join(OUT, f"header-case-lid-{deg}-{half}.svg"), "w", encoding="utf-8", newline="\n").write(art.svg(note))
    return {"deg": deg, "top": round(sil.bounds[1], 2), "hinge": round(hy0, 2), "rim_front": round(rim_front, 2)}


if __name__ == "__main__":
    for d in ANGLES:
        print(build(d))
