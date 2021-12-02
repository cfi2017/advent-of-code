from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=11)
data = puzzle.input_data
import time


def next_grid(g):
    ng = []
    for x in range(len(g)):
        next_row = ''
        for y in range(len(g[0])):
            if g[x][y] == 'L' and get_surrounding(g, x, y) == 0:
                next_row += '#'
            elif g[x][y] == '#' and get_surrounding(g, x, y) >= 5:
                next_row += 'L'
            else:
                next_row += g[x][y]
        ng.append(next_row)
    return ng


def get_surrounding(g, center_x, center_y):
    vals = []
    # return [surrounding_recursive(g, center_x, center_y, x, y)
    # for x in [-1, 0, 1] for y in [-1, 0, 1] if not (x == y == 0)].count(True)
    #
    for x in [-1, 0, 1]:
        for y in [-1, 0, 1]:
            if x == y == 0:
                continue

            # if 0 <= center_x+x < len(grid) and 0 <= center_y+y < len(grid[0]):
            #     vals.append(grid[center_x+x][center_y+y])

            # i = 1
            # while 0 <= center_x + i*x < len(g) and 0 <= center_y + i*y < len(g[0]):
            #     var = g[center_x + i * x][center_y + i * y]
            #     if var != '.':
            #         vals.append(var)
            #         break
            #     i += 1

            vals.append(surrounding_recursive(g, center_x, center_y, x, y))
    return vals.count(True)


def surrounding_recursive(g, x, y, direction_x, direction_y, cache=None):
    if cache is None:
        cache = {}
    c_key = (x, y, direction_x, direction_y)
    if c_key in cache:
        return cache[c_key]
    check_x = x + direction_x
    check_y = y + direction_y
    if 0 <= check_x < len(g) and 0 <= check_y < len(g[0]):
        val = g[check_x][check_y]
        if val != '.':
            cache[c_key] = val == '#'
            return cache[c_key]
        else:
            rec = surrounding_recursive(g, check_x, check_y, direction_x, direction_y, cache)
            cache[c_key] = rec
            return rec
    else:
        cache[c_key] = False
        return False


def occupied(g):
    count = 0
    for line in g:
        count += line.count('#')
    return count


grid = list(map(list, data.splitlines()))


def part_2(g):
    while True:
        next = next_grid(g)
        if g == next:
            break
        g = next
    return occupied(g)

import sys
sys.setrecursionlimit(10000)

val = part_2(grid)
print(val)
# puzzle.answer_b = val

time_total = 0
test_count = 10
for temp_step in range(test_count):
    time_before = time.time()
    part_2(grid)
    time_total += time.time() - time_before
print(test_count, "trials of part 2 took", time_total / test_count)
time_total = 0
