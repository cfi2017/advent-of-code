import math
import operator

from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=20)
from functools import reduce
data = puzzle.input_data

data = open('sample.txt').read()
size = 3


def parse(d):
    d = d.replace('#', '1')
    d = d.replace('.', '0')
    d = d.split('\n\n')
    tiles = {}
    for p in d:
        p = p.splitlines()
        id = int(p[0][5:-1])
        tile = p[1:]
        tiles[id] = tile
    return tiles


def rotate90(img):
    return [list(reversed(x)) for x in zip(*img)]


def flip(img):
    return [list(reversed(x)) for x in img]


def get_borders(tile):
    borders = []
    borders.append(int(''.join(tile[0]), 2))
    borders.append(int(''.join(tile[0][::-1]), 2))
    tile = rotate90(tile)
    borders.append(int(''.join(tile[0]), 2))
    borders.append(int(''.join(tile[0][::-1]), 2))
    tile = rotate90(tile)
    borders.append(int(''.join(tile[0]), 2))
    borders.append(int(''.join(tile[0][::-1]), 2))
    tile = rotate90(tile)
    borders.append(int(''.join(tile[0]), 2))
    borders.append(int(''.join(tile[0][::-1]), 2))
    rotate90(tile)
    return borders


def calc_borders(tiles):
    borders = {}
    for id in tiles:
        borders[id] = get_borders(tiles[id])
    return borders


def bordering_ids(borders, current, arr):
    return [x for x in arr if any([True if x in borders[current] else False for x in borders[x]])]


def get_placements_frame(borders, corners, edges):
    found = [corners[0]]
    grid = [[None] * size for i in range(size)]
    grid[0][0] = corners[0]
    pos = (0, 0)
    direction = (1, 0)
    while True:
        curr = grid[pos[1]][pos[0]]
        bordering_edges = [n for n in bordering_ids(borders, curr, edges) if n not in found]
        bordering_corner = [n for n in bordering_ids(borders, curr, corners) if n not in found]
        pos = tuple(map(operator.add, pos, direction))
        if len(bordering_edges) >= 1:
            found.append(bordering_edges[0])
            grid[pos[1]][pos[0]] = bordering_edges[0]
        elif len(bordering_corner) == 1:
            found.append(bordering_corner[0])
            grid[pos[1]][pos[0]] = bordering_corner[0]
            direction = (-direction[1], direction[0])
        else:
            break
    return grid


def get_placements(borders, grid, inner):
    found = []
    for i1 in range(len(grid)):
        line = grid[i1]
        if None in line:
            for i in range(len(line)):
                if line[i] is not None:
                    continue
                candidates = [n for n in bordering_ids(borders, line[i-1], inner) if n not in found]
                for candidate in candidates:
                    if any([True if x in borders[candidate] else False for x in borders[grid[i1-1][i]]]):
                        found.append(candidate)
                        grid[i1][i] = candidate
                        break
                if grid[i1][i] is None:
                    print("error")
    return grid


def find_tile_rotation(borders, tiles, id, compareto, previous):
    # if previous = true, left match
    # else right match
    border = [x for x in borders[id] if x in borders[compareto]][0]
    tile = tiles[id]
    while True:
        if int(''.join(tile[0]), 2) == border:
            break
        tile = flip(tile)
        if int(''.join(tile[0]), 2) == border:
            break
        tile = flip(tile)
        tile = rotate90(tile)

    tile = flip(tile)
    tile = rotate90(tile)
    if previous:
        tile = rotate90(tile)
        tile = rotate90(tile)
    return tile


def place_grid(borders, tiles, grid):
    ng = [[None] * size for i in range(size)]
    for y in range(size):
        for x in range(size):
            if x == 0:
                previous = False
                check = grid[y][x + 1]
            else:
                check = grid[y][x - 1]
                previous = True
            ng[y][x] = find_tile_rotation(borders, tiles, grid[y][x], check, previous)
    return ng


def rotate(origin, point, angle):
    """
    Rotate a point counterclockwise by a given angle around a given origin.

    The angle should be given in radians.
    """
    ox, oy = origin
    px, py = point

    qx = ox + math.cos(angle) * (px - ox) - math.sin(angle) * (py - oy)
    qy = oy + math.sin(angle) * (px - ox) + math.cos(angle) * (py - oy)
    return qx, qy


data = parse(data)
borders = calc_borders(data)

print(data)
print(borders)


def cut_border(img):
    return list(map(lambda x: x[1:-1], img[1:-1]))


def stitch_image(grid):
    img = [[None] * size * 8 for i in range(size * 8)]
    for y in range(len(grid)):
        for x in range(len(grid)):
            tile = cut_border(grid[y][x])
            for y2 in range(8):
                for x2 in range(8):
                    img[y * 8 + y2][x * 8 + x2] = tile[y2][x2]
    return img


def matches(borders, i):
    edges = borders[i]
    m = 0
    for b in borders:
        if b == i:
            continue
        if any([True if e in edges else False for e in borders[b]]):
            m += 1
            continue
    return m


pattern = """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """
pattern = pattern.splitlines()


def monster_checkline(line, offset, pattern):
    for i in range(len(pattern)):
        if pattern[i] == ' ':
            continue
        else:
            if pattern[i] != line[offset + i]:
                return False
    return True


def is_monster(img, x, y):
    for i in range(3):
        if not monster_checkline(img[y+i], x, pattern[i]):
            return False
    return True


def monster_count(img):
    count = 0
    for y in range(len(img) - 3):
        for x in range(len(img) - len(pattern[0])):
            if is_monster(img, x, y):
                count += 1
    return count


corners = [i for i in borders if matches(borders, i) == 2]
print(corners)
res = reduce((lambda x, y: x * y), corners)
print(res)
# puzzle.answer_a = res

print(len(borders))

edges = [i for i in borders if matches(borders, i) == 3]
inner = [i for i in borders if matches(borders, i) == 4]

frame = get_placements_frame(borders, corners, edges)
print(frame)
grid = get_placements(borders, frame, inner)
print(grid)
grid = place_grid(borders, data, grid)
print(grid)
img = stitch_image(grid)
for i in range(len(img)):
    line = img[i]
    line = ''.join(line)
    line = line.replace('0', '.')
    line = line.replace('1', '#')
    img[i] = line
    print(line)

for i in range(4):
    if monster_count(img) > 0:
        print('monsters found, nonflipped', i)
        break

    img = flip(img)

    if monster_count(img) > 0:
        print('monsters found, flipped', i)
        break

    img = flip(img)
    img = rotate90(img)
