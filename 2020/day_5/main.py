from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=5)
i = puzzle.input_data.splitlines()

s = {int(''.join(['0' if c in 'FL' else '1' for c in x]), 2) for x in i}
print(max(s), [i+1 for i in s if i+1 not in s and i+2 in s][0])
