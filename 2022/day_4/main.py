import numpy as np
from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=4)
data = puzzle.input_data


def contains(arr, other):
    for e in arr:
        if e not in other:
            return False
    return True


def contains_any(arr, other):
    for e in arr:
        if e in other:
            return True
    return False


a = 0
b = 0
for line in data.splitlines():
    elves = line.split(',')
    [l, r] = list(map(lambda x: range(x[0], x[1]+1), map(lambda e: list(map(int, e.split('-'))), elves)))
    if contains(l, r) or contains(r, l):
        a += 1
    if contains_any(l, r) or contains_any(r, l):
        b += 1


print(a)
puzzle.answer_a = a
print(b)
puzzle.answer_b = b

