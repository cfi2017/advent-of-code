from aocd.models import Puzzle
from aocd import submit
from functools import reduce

puzzle = Puzzle(year=2021, day=4)
data = puzzle.input_data
lines = list(filter(lambda x: x != "", data.splitlines()))

result_a = 0
result_b = 0

sequence = list(map(int, lines[0].split(',')))

lines = list(map(lambda x: list(map(int, x.lstrip(' ').replace('  ', ' ').split(' '))), lines[1:]))

print(lines)
wbs = []
for seq in sequence:
    for line in lines:
        for index, num in enumerate(line):
            if num == seq:
                line[index] = num + .1
    # check lines
    w = 'none'
    wb = -1
    for i in range(0, 500, 5):
        for line in lines[i:i+5]:
            if all(map(lambda x: isinstance(x, float), line)):
                # line is winner
                if i not in wbs:
                    wb = i
                    break
        for ci in range(5):
            column = list(map(lambda x: x[ci], lines[i:i+5]))
            if all(map(lambda x: isinstance(x, float), column)):
                if i not in wbs:
                    wb = i
                    break
        if wb != -1:
            break
    if wb != -1:
        wbs.append(wb)
        print(wb)
        cur_win = sum([sum([x for x in line if isinstance(x, int)]) for line in lines[wb:wb + 5]]) * seq
        if result_a == 0:
            result_a = cur_win
        if cur_win != 0:
            result_b = cur_win

print(lines)
# result_a = sum([lines[i] < num for i, num in enumerate(lines[1:])])
# result_b = sum([sum(lines[i:i + 3]) < sum(lines[i + 1:i + 4]) for i, num in enumerate(lines[:-3])])

submit(result_a, part="a", year=2021, day=4)
submit(result_b, part="b", year=2021, day=4)

