#!/usr/bin/env python3
"""
allows quick sneak peek into whats happening with rnapkin
print points and pipe it in
cargo run | atelier/plot.py
"""
import sys
import math
from pathlib import Path

import matplotlib
import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
from collections import defaultdict
import re

script_dir = Path(__file__).parent

pex = re.compile(r"x: ([-\d\.e]+).*?y: ([-\d\.e]+)")
# doing it separetely for convenience
ntex = re.compile(r"nt: ([AUGCaugc])")
matplotlib.rcParams["figure.figsize"] = [6.4 * 1.8, 4.8 * 1.8]  # (default: [6.4, 4.8])
matplotlib.rcParams["figure.dpi"] = 100  # (default: 100.0)k


COLORS = {"A": "gold", "C": "forestgreen", "G": "firebrick", "U": "royalblue"}
LIMS = 20


def draw_circles(ax, points, c="b", radius=0.5, **kwargs):
    circles = [
        plt.Circle((xi, yi), radius=radius, linewidth=0, **kwargs) for xi, yi in points
    ]
    cc = matplotlib.collections.PatchCollection(circles)
    cc.set_facecolor(c)
    cc.set(**kwargs)
    return ax.add_collection(cc)


def main():
    if sys.stdin.isatty():
        return

    points = []
    ntpoints = defaultdict(list)
    for line in sys.stdin:
        try:
            result = pex.search(line)
            nt = ntex.search(line).group(1)
            point = float(result.group(1)), float(result.group(2))
            if nt is not None:
                ntpoints[nt.upper()].append(point)
                continue
            points.append(point)
        except AttributeError:
            print("not a point", file=sys.stderr)

    # don't blame me, python allowed me to do it:
    dumb = [*points, *(i for ntlist in ntpoints.values() for i in ntlist)]

    if not dumb:
        print("nothing to draw", file=sys.stderr)
        return

    fig, ax = plt.subplots()
    ax.set_aspect("equal")
    ax.set_axisbelow(True)
    ax.grid(linestyle="--")

    draw_circles(ax, points, "deeppink", radius=0.5)

    handles = []
    for k, v in ntpoints.items():
        if v:
            draw_circles(ax, v, COLORS[k], radius=0.5, label=k)
            handles.append(mpatches.Patch(color=COLORS[k], label=k))
            for p in v:
                print(k, p)

    bx, by = float("-INF"), float("-INF")
    sx, sy = float("INF"), float("INF")

    for x, y in dumb:
        bx, by = max(bx, x), max(by, y)
        sx, sy = min(sx, x), min(sy, y)


    buffer = 2
    bx, by, = math.ceil(bx), math.ceil(by)
    sx, sy, = math.ceil(sx), math.ceil(sy)
    ax.set_xlim(sx - buffer, bx + buffer)
    ax.set_ylim(sy - buffer, by + buffer)
    # ax.set_xlim(-LIMS, LIMS)
    # ax.set_ylim(-LIMS, LIMS)
    ax.legend(handles=handles)

    fig.patch.set_facecolor("#393E46")
    ax.patch.set_facecolor("#393E46")
    plt.savefig(script_dir / "fig.gi.png")
    plt.show()


if __name__ == "__main__":
    main()
