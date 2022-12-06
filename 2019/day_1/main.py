import math

from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=1)
data = puzzle.input_data


fuel_calc = lambda x: math.floor(x / 3) - 2

def recursive_calc(x):
    total = fuel_calc(x)
    x = total
    while x > 0:
        x = fuel_calc(x)
        if x < 0:
            break
        total += x
    return total


assert fuel_calc(12) == 2
assert fuel_calc(14) == 2
assert fuel_calc(1969) == 654
assert fuel_calc(100756) == 33583

a = sum([fuel_calc(int(l)) for l in data.splitlines()])
print(a)
puzzle.answer_a = a

b = sum([recursive_calc(int(l)) for l in data.splitlines()])
print(b)
puzzle.answer_b = b



