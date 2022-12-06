from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=5)
data = puzzle.input_data

[start, instr] = data.split('\n\n')
start = list(reversed(start.splitlines()))[1:]
stacks = [[], [], [], [], [], [], [], [], []]
for line in start:
    print(line)
    stack = 0
    i = 0
    while i < len(line):
        c = line[i]
        if c not in ['[', ']', ' ']:
            stacks[stack].append(c)
        if c == ' ' and (i + 1) % 4 == 0:
            stack += 1
        i += 1

print(stacks)

for line in instr.splitlines():
    print(line)
    print(stacks)
    [_, count, _, fromindex, _, to] = line.split(' ')
    count = int(count)
    fromindex = int(fromindex) - 1
    to = int(to) - 1
    for _ in range(count):
        n = stacks[fromindex].pop()
        stacks[to].append(n)

result = ""
for stack in stacks:
    n = stack.pop()
    result += n

print(result)
puzzle.answer_a = result

[start, instr] = data.split('\n\n')
start = list(reversed(start.splitlines()))[1:]
stacks = [[], [], [], [], [], [], [], [], []]
for line in start:
    print(line)
    stack = 0
    i = 0
    while i < len(line):
        c = line[i]
        if c not in ['[', ']', ' ']:
            stacks[stack].append(c)
        if c == ' ' and (i + 1) % 4 == 0:
            stack += 1
        i += 1

print(stacks)

for line in instr.splitlines():
    #print(line)
    #print(stacks)
    [_, count, _, fromindex, _, to] = line.split(' ')
    count = int(count)
    fromindex = int(fromindex) - 1
    to = int(to) - 1
    n = stacks[fromindex][-count:]
    #print(f"moving {n} from {fromindex} to {to}")
    stacks[to] += n
    stacks[fromindex] = stacks[fromindex][:-count]

result = ""
for stack in stacks :
    if len(stack) > 0:
        n = stack.pop()
        result += n

print(result)
puzzle.answer_b = result
