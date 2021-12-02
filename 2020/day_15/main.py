from aocd.models import Puzzle
import matplotlib.pyplot as plt
puzzle = Puzzle(year=2020, day=15)
data = puzzle.input_data
data = list(map(int, data.split(',')))

mem = data.copy()
# mem = [0,3,6]

i = len(mem)
while True:
    n = mem[-1]
    if mem.count(n) == 1:
        mem.append(0)
    else:
        last = i - 1 - (mem[:-1][::-1].index(n) + 1)
        mem.append((i - 1) - last)
    i += 1
    if i == 2020:
        break

print(mem[-1])
# puzzle.answer_a = mem[-1]

stack = data.copy()
stack.reverse()
mem = {}
val = 0
for i in range(30000000):
    if len(stack) > 0:
        val = stack.pop()
    elif len(mem[val]) == 1:
        val = 0
    else:
        val = mem[val][-1] - mem[val][-2]

    if val in mem:
        mem[val].append(i)
    else:
        mem[val] = [i]

print(val)
puzzle.answer_b = val
