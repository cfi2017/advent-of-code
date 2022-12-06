from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=3)
data = puzzle.input_data

print(data)

MODIFIERS = {
    'U': (0, 1),
    'D': (0, -1),
    'L': (-1, 0),
    'R': (1, 0),
}


def get_intersections(a, b):
    points = []
    for point_a in a:
        for point_b in b:
            if point_a == point_b:
                points.append(point_a)
                break
    return points

def get_points(turns):
    points = []
    pos = (0, 0)

    for turn in turns:
        direction = MODIFIERS[turn[0]]
        amount = int(turn[1:])
        for _ in range(amount):
            pos = (pos[0] + direction[0], pos[1] + direction[1])
            points.append(pos)

    return points


wire_turns = [line.split(',') for line in data.splitlines()]
wires = list(map(get_points, wire_turns))
print(len(wires[0]))
print(len(wires[1]))

intersections = get_intersections(wires[0], wires[1])
print(len(intersections))
