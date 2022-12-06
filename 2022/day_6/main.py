from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=6)
data = puzzle.input_data

from itertools import islice
def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def find_window(stream, size):
    for i, buffer in enumerate(window(stream, size)):
        if len(set(buffer)) == size:
            return i + size


puzzle.answer_a = find_window(data, 4)
puzzle.answer_b = find_window(data, 14)
