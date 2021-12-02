from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=17)
data = puzzle.input_data


def init(data):
    g = set()
    for y in range(len(data)):
        line = data[y]
        for x in range(len(line)):
            coord = (x, y, 0, 0)
            if line[x] == '#':
                g.add(coord)
    return g


def p(g):
    print(list(g.values()).count('#'))


def active_count(active, c):
    n = 0
    for x in [-1, 0, 1]:
        for y in [-1, 0, 1]:
            for z in [-1, 0, 1]:
                for w in [-1, 0, 1]:
                    if x == y == z == w == 0:
                        continue
                    coord = (c[0]+x, c[1]+y, c[2]+z, c[3]+w)
                    if coord in active:
                        n += 1
    return n

def run(d):
    active = init(d)
    for _ in range(6):
        next = set()
        for x in range(min(coord[0] for coord in active)-1, max(coord[0] for coord in active)+2):
            for y in range(min(coord[1] for coord in active)-1, max(coord[1] for coord in active)+2):
                for z in range(min(coord[2] for coord in active)-1, max(coord[2] for coord in active)+2):
                    for w in range(min(coord[3] for coord in active)-1, max(coord[3] for coord in active)+2):
                        c = active_count(active, (x, y, z, w))
                        if (x, y, z, w) not in active and c == 3:
                            next.add((x, y, z, w))
                        elif (x, y, z, w) in active and 2 <= c <= 3:
                            next.add((x, y, z, w))
        active = next
    return len(active)


# data = """.#.
# ..#
# ###"""

data = data.splitlines()

count = run(data)
print(count)
puzzle.answer_b = count
