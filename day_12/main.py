from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=12)
data = puzzle.input_data


# data = """F10
# N3
# F7
# R90
# F11"""


def parse(line):
    ins = line[0]
    args = int(line[1:])
    return (ins, args)


def move(pos, facing, line):
    [ins, args] = parse(line)
    if ins == 'N':
        pos[0] -= args
    elif ins == 'S':
        pos[0] += args
    elif ins == 'E':
        pos[1] += args
    elif ins == 'W':
        pos[1] -= args
    elif ins == 'L':
        facing -= args
        if facing < 0:
            facing += 360
    elif ins == 'R':
        facing += args
        if facing >= 360:
            facing -= 360
    elif ins == 'F':
        if facing == 0:
            pos[0] -= args
        elif facing == 90:
            pos[1] += args
        elif facing == 180:
            pos[0] += args
        elif facing == 270:
            pos[1] -= args
        else:
            print('error')
    else:
        print('err2')
    print(ins, args, pos, facing)
    return pos, facing


pos = [0, 0]
facing = 90

for line in data.splitlines():
    pos, facing = move(pos, facing, line)
    # print(pos, facing)

distance = abs(pos[0]) + abs(pos[1])
print(distance)
puzzle.answer_a = distance

pos = [0, 0]
pos_w = [-1, 10]


def move2(pos, pos_w, line):
    [ins, args] = parse(line)
    if ins == 'N':
        pos_w[0] -= args
    elif ins == 'S':
        pos_w[0] += args
    elif ins == 'E':
        pos_w[1] += args
    elif ins == 'W':
        pos_w[1] -= args
    elif ins == 'L':
        for i in range(args//90):
            pos_w = [-pos_w[1], pos_w[0]]
    elif ins == 'R':
        for i in range(args//90):
            pos_w = [pos_w[1], -pos_w[0]]
    elif ins == 'F':
        pos[0] += args * pos_w[0]
        pos[1] += args * pos_w[1]
    else:
        print('err2')
    print(ins, args, pos, pos_w)
    return pos, pos_w


for line in data.splitlines():
    pos, pos_w = move2(pos, pos_w, line)
    # print(pos, pos_w)

distance = abs(pos[0]) + abs(pos[1])
print(distance)
puzzle.answer_b = distance
