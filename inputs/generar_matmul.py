#!/usr/bin/env -S uv run
import random
import json
import sys


WIDTH = 1500
HEIGHT = 1000
CANTIDAD = WIDTH * HEIGHT
RANGO_MIN = 0
RANGO_MAX = 100000

print("Shuffling A", file=sys.stderr)
a = [random.random() * RANGO_MAX + RANGO_MIN for _ in range(CANTIDAD)]
print("Shuffling B", file=sys.stderr)
b = [random.random() * RANGO_MAX + RANGO_MIN for _ in range(CANTIDAD)]
# random.shuffle(a)
# random.shuffle(b)

print("Preparing test dict", file=sys.stderr)
name = f"matmul{WIDTH}x{HEIGHT}"
test = {
  "fun": "matrix_multiply",
  "name": name,
  "type": 4,
  "executions": 100,
  "arg": {
    "a": a,
    "b": b,
    "width": WIDTH,
  },
}

print("Generating input file", file=sys.stderr)
with open(f"inputs/{name}.json", "w") as f:
  json.dump(test, f)
