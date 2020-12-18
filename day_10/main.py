from aocd.models import Puzzle
from functools import lru_cache
puzzle = Puzzle(year=2020, day=10)
lines = puzzle.input_data

# lines = """28
# 33
# 18
# 42
# 31
# 14
# 46
# 20
# 48
# 47
# 24
# 23
# 49
# 45
# 19
# 38
# 39
# 11
# 1
# 32
# 25
# 35
# 8
# 17
# 7
# 9
# 4
# 2
# 34
# 10
# 3"""


lines = lines.splitlines()
lines = list(map(int, lines))

lines = sorted(lines)

current = 0
ones = 0
threes = 0
device = max(lines) + 3

for line in lines:
    if line > current + 3:
        # invalid configuration
        print('cant go higher than ', line)
        break

    if line == current + 3:
        threes += 1

    if line == current + 1:
        ones += 1

    current = line

print((threes + 1) * ones)

cache = {}


def recurse(curr, pos, dev):
    global cache
    branch = curr * len(lines) + pos
    if branch in cache:
        return cache[branch]
    if pos >= len(lines):
        return 1
    if lines[pos] > curr + 3:
        return 0
    if lines[pos] >= dev - 3:
        return 1
    curr = lines[pos]
    ps = recurse(curr, pos + 1, dev) + recurse(curr, pos + 2, dev) + recurse(curr, pos + 3, dev)
    cache[branch] = ps
    return ps


paths = recurse(0, 0, device)
print(paths)
puzzle.answer_b = paths
