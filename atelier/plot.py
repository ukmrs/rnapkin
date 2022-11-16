#!/usr/bin/env python3
import sys
from pathlib import Path

import matplotlib
import matplotlib.pyplot as plt
import re

script_dir = Path(__file__).parent

pex = re.compile(r"x: ([-\d\.]+).*?y: ([-\d\.]+)")
matplotlib.rcParams["figure.figsize"] = [6.4 * 1.8, 4.8 * 1.8]  # (default: [6.4, 4.8])
matplotlib.rcParams["figure.dpi"] = 100  # (default: 100.0)k

LIMS = 14


def draw_circles(ax, points, c="b", radius=0.5, **kwargs):
    circles = [
        plt.Circle((xi, yi), radius=radius, linewidth=0, **kwargs) for xi, yi in points
    ]
    cc = matplotlib.collections.PatchCollection(circles)
    cc.set_facecolor(c)
    cc.set(**kwargs)
    ax.add_collection(cc)


def main():
    if sys.stdin.isatty():
        return

    points = []
    for line in sys.stdin:
        try:
            result = pex.search(line)
            point = float(result.group(1)), float(result.group(2))
            points.append(point)
        except AttributeError:
            print("not a point", file=sys.stderr)

    if not points:
        print("nothing to draw", file=sys.stderr)
        return

    fig, ax = plt.subplots()
    ax.set_aspect("equal")
    ax.set_axisbelow(True)
    ax.grid(linestyle="--")

    draw_circles(ax, points, "deeppink")
    # ax.set_xlim(-LIMS, LIMS)
    # ax.set_ylim(-LIMS, LIMS)
    ax.set_xlim(-LIMS, LIMS)
    ax.set_ylim(-LIMS, LIMS)

    fig.patch.set_facecolor("#393E46")
    ax.patch.set_facecolor("#393E46")
    plt.savefig(script_dir / "fig.gi.png")
    plt.show()



if __name__ == "__main__":
    main()
