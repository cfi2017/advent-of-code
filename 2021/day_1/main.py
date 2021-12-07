from aocd.models import Puzzle
from aocd import submit
from functools import reduce

YEAR = 2021
DAY = 2
puzzle = Puzzle(year=YEAR, day=DAY)
data = puzzle.input_data
lines = data.splitlines()

conversions = {
    'forward': lambda x, h, v, v2, a: (h + x, v, v_2 + x * a, a),
    'up': lambda x, h, v, v2, a: (h, v - x, v_2, a - x),
    'down': lambda x, h, v, v2, a: (h, v + x, v_2, a + x),
}

if __name__ == '__main__':
    h, v, v_2, aim = 0, 0, 0, 0
    for line in lines:
        k, x = line.split(' ')
        h, v, v_2, aim = conversions[k](int(x), h, v, v_2, aim)
    result_a = v * h
    result_b = v_2 * h
    submit(result_a, part="a", year=YEAR, day=DAY)
    submit(result_b, part="b", year=YEAR, day=DAY)

