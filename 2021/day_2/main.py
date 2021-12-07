from aocd.models import Puzzle
from aocd import submit

puzzle = Puzzle(year=2021, day=2)
data = puzzle.input_data
lines = list(map(int, data.splitlines()))

result_a = 0
result_b = 0

# result_a = sum([lines[i] < num for i, num in enumerate(lines[1:])])
# result_b = sum([sum(lines[i:i + 3]) < sum(lines[i + 1:i + 4]) for i, num in enumerate(lines[:-3])])

submit(result_a, part="a", year=2021, day=1)
submit(result_b, part="b", year=2021, day=1)

