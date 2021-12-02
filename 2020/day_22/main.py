import time
from copy import deepcopy, copy
from functools import reduce
import numpy as np

from aocd.models import Puzzle

puzzle = Puzzle(year=2020, day=22)
data = puzzle.input_data
# data = open('mingmingrr.txt').read()

p1, p2 = data.split('\n\n')
p1 = list(map(int, p1.splitlines()[1:]))
p2 = list(map(int, p2.splitlines()[1:]))


def is_infinite(decks, caches):
    for i in [0, 1]:
        d = tuple(decks[i])
        if d in caches[i]:
            return True
        caches[i].add(d)
    return False


def append(arrs, nums, p1):
    if p1:
        arrs[0] += nums
    else:
        arrs[1] += nums[::-1]


def rec(p1, p2):
    c1 = set()
    c2 = set()
    while True:
        if is_infinite([p1, p2], [c1, c2]):
            return True
        if len(p1) == 0 or len(p2) == 0:
            return len(p2) == 0

        n1, n2 = p1.pop(0), p2.pop(0)
        if len(p1) < n1 or len(p2) < n2:
            append([p1, p2], [n1, n2], n1 > n2)
        else:
            append([p1, p2], [n1, n2], rec(copy(p1[:n1]), copy(p2[:n2])))


def part1(p1, p2):
    while True:
        n1, n2 = p1.pop(0), p2.pop(0)
        if n1 > n2:
            p1.append(n1)
            p1.append(n2)
        elif n1 < n2:
            p2.append(n2)
            p2.append(n1)
        else:
            print('tie')

        if len(p1) == 0 or len(p2) == 0:
            break

    return reduce(lambda x, y: x + y, [n * (i + 1) for i, n in enumerate((p1 + p2)[::-1])])


def part2(p1, p2):
    rec(p1, p2)
    return reduce(lambda x, y: x + y, [n * (i + 1) for i, n in enumerate((p1 + p2)[::-1])])


original = copy(p1), copy(p2)
print('part 1', part1(p1, p2))
p1, p2 = copy(original[0]), copy(original[1])
print('part 2', part2(p1, p2))

time_total = 0
test_count = 10
for temp_step in range(test_count):
    cache = {}
    time_before = time.time()
    p1, p2 = copy(original[0]), copy(original[1])
    part2(p1, p2)
    time_total += time.time() - time_before
print(test_count, "trials averaged", time_total / test_count)


