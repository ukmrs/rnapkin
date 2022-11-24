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
from dataclasses import dataclass

script_dir = Path(__file__).parent

matplotlib.rcParams["figure.figsize"] = [6.4 * 1.8, 4.8 * 1.8]  # (default: [6.4, 4.8])
matplotlib.rcParams["figure.dpi"] = 100  # (default: 100.0)k


COLORS = {"A": "gold", "C": "forestgreen", "G": "firebrick", "U": "royalblue"}


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

    bubbles = defaultdict(list)

    for line in sys.stdin:
        x, y, nt, _ = line.split(",")
        bubbles[nt].append((float(x), float(y)))

    if not bubbles:
        print("nothing to draw", file=sys.stderr)
        sys.exit(1)

    fig, ax = plt.subplots()
    ax.set_aspect("equal")
    ax.set_axisbelow(True)
    ax.grid(linestyle="--")

    handles = []
    for k, v in bubbles.items():
        if not v:
            continue

        draw_circles(ax, v, COLORS[k], radius=0.5, label=k, ec="black")
        handles.append(mpatches.Patch(color=COLORS[k], label=k))

    bx, by = float("-INF"), float("-INF")
    sx, sy = float("INF"), float("INF")

    for bbllist in bubbles.values():
        for x, y in bbllist:
            bx, by = max(bx, x), max(by, y)
            sx, sy = min(sx, x), min(sy, y)

    buffer = 2
    bx, by, = math.ceil(
        bx
    ), math.ceil(by)
    sx, sy, = math.ceil(
        sx
    ), math.ceil(sy)
    ax.set_xlim(sx - buffer, bx + buffer)
    ax.set_ylim(sy - buffer, by + buffer)
    ax.legend(handles=handles)

    fig.patch.set_facecolor("#393E46")
    ax.patch.set_facecolor("#393E46")

    plt.savefig(script_dir / "fig.gi.png")
    plt.show()


if __name__ == "__main__":
    main()
