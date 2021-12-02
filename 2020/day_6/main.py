from aocd.models import Puzzle

puzzle = Puzzle(year=2020, day=6)
input = puzzle.input_data

input = input.split('\n\n')


# input = [x.splitlines() for x in input]

print(sum(len(set(x) - {'\n'}) for x in input))
print(sum(len(set.intersection(*(set(y) for y in x.split()))) for x in input))


