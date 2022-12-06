from numpy import array_split
from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=3)
data = puzzle.input_data
data = list(map(lambda l: [x - 65 + 27 if x <= 90 else x - 96 for x in [ord(c) for c in [*l]]], data.splitlines()))
union = lambda l, r: [c for c in l for cb in r if c == cb]
a = sum([union(a, b)[0] for [a, b] in [array_split(s, 2) for s in data]])
b = sum([union(union(a, b), c)[0] for a, b, c in zip(*[iter(data)]*3)])
print(a, b)
puzzle.answer_a = a
puzzle.answer_b = b
