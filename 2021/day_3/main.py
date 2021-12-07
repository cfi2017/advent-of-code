from aocd.models import Puzzle
from aocd import submit
from functools import reduce

puzzle = Puzzle(year=2021, day=3)
data = puzzle.input_data
lines = data.splitlines()


def most_common(arr, i):
    """Returns the most common bit for an index position for every string in the array."""
    return '1' if sum(map(lambda x: x[i] == '1', arr)) >= len(arr) / 2 else '0'


def least_common(arr, i):
    """Returns the least common bit for an index position for every string in the array."""
    return '1' if sum(map(lambda x: x[i] == '1', arr)) < len(arr) / 2 else '0'


def bti(bin):
    """Returns the integer representation of a given binary string."""
    return int(bin, 2)


def solve_a():
    gamma = bti(''.join([most_common(lines, i) for i in range(12)]))
    epsilon = bti(''.join([least_common(lines, i) for i in range(12)]))
    return gamma * epsilon


def accessor(i, bit):
    """Returns a lambda function used to filter an array based on the given position and bit."""
    return lambda x: x[i] == bit


def progressive_filter(f):
    """Returns a lambda function used to filter an array based on the given position
    or returns the list if it only has one entry."""
    return lambda arr, i: arr if len(arr) == 1 else list(filter(accessor(i, f(arr, i)), arr))


def solve_b():
    oxygen = bti(reduce(progressive_filter(most_common), range(12), lines)[0])
    co2 = bti(reduce(progressive_filter(least_common), range(12), lines)[0])
    return oxygen * co2


results = (solve_a(), solve_b())

result_a = results[0]
result_b = results[1]

submit(result_a, part="a", year=2021, day=3)
submit(result_b, part="b", year=2021, day=3)
