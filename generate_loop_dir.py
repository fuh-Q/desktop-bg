"""
Essentially with the CLI there's a loop flag that allows you to specify
a directory with pre-generated images of every minute to loop through

This script is pretty much just to generate such a directory
"""

import os
import sys
import subprocess

from pathlib import Path

if not len(sys.argv) - 1:
    print("Directory not provided")
    sys.exit(1)

target = Path.cwd() / sys.argv[1]

try:
    os.makedirs(target, exist_ok=True)
except PermissionError as e:
    print(f"Failed creating {e.filename} due to a permission error")
    sys.exit(1)

project_dir = Path(__file__).parent
def make_image(hour: int, minute: int):
    hour, minute = str(i), str(j)
    args = [
        str(project_dir / "target" / "release" / "desktop-bg.exe"),
        "-t", str(target / f"{hour}-{minute}.png"),
        "-H", hour,
        "-M", minute,
    ]

    p = subprocess.Popen(args)
    out, err = p.communicate()

    if out:
        print(out.decode("utf-8"))
    if err:
        print(err.decode("utf-8"))

try:
    for i in range(24):
        for j in range(60):
            make_image(i, j)
except KeyboardInterrupt:
    print("Stopping...")
