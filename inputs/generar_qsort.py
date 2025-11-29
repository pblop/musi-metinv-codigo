#!/usr/bin/env -S uv run
import random
import json
import sys


CANTIDAD = 25_000_000
RANGO_MIN = 0
RANGO_MAX = 100000

numeros = list(range(CANTIDAD))
random.shuffle(numeros)

qs_name = f"quicksort_{CANTIDAD // 1_000_000}m"
test = {
  "fun": "quicksort",
  "name": qs_name,
  "type": 2,
  "executions": 100,
  "arg": numeros,
}

with open(f"inputs/{qs_name}.json", "w") as f:
  json.dump(test, f)
