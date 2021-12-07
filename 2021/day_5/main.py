from aocd.models import Puzzle
from aocd import submit

puzzle = Puzzle(year=2021, day=5)
data = puzzle.input_data

lines = data.splitlines()

result_a = 0
result_b = 0

points_b = {}
points = {}
for line in lines:
    line = list(map(lambda x: list(map(int, x.split(','))), line.split(' -> ')))
    x1, y1 = line[0]
    x2, y2 = line[1]
    if x1 == x2:  # line on x axis
        for y in range(min(y1, y2), max(y1, y2) + 1):
            if not (x1, y) in points:
                points[(x1, y)] = 0
            points[(x1, y)] += 1
            if not (x1, y) in points_b:
                points_b[(x1, y)] = 0
            points_b[(x1, y)] += 1
    elif y1 == y2:  # line on y axis
        for x in range(min(x1, x2), max(x1, x2) + 1):
            if not (x, y1) in points:
                points[(x, y1)] = 0
            points[(x, y1)] += 1
            if not (x, y1) in points_b:
                points_b[(x, y1)] = 0
            points_b[(x, y1)] += 1
    else:
        if (x2 < x1 and not y2 < y1) or (y2 < y1 and not x2 < x1):
            x, y = max(x1, x2), min(y1, y2)
            while x != min(x1, x2) -1 and y != max(y1, y2) + 1:
                if not (x, y) in points_b:
                    points_b[(x, y)] = 0
                points_b[(x, y)] += 1
                x -= 1
                y += 1
        else:
            x, y = min(x1, x2), min(y1, y2)
            while x != max(x1, x2) + 1 and y != max(y1, y2) + 1:
                if not (x, y) in points_b:
                    points_b[(x, y)] = 0
                points_b[(x, y)] += 1
                x += 1
                y += 1

s = 0
for v, k in enumerate(points):
    if points[k] > 1:
        s += 1
print(s)
for v, k in enumerate(points_b):
    if points_b[k] > 1:
        result_b += 1
print(result_b)
print(points_b)
submit(s, part="a", year=2021, day=5)
submit(result_b, part="b", year=2021, day=5)

