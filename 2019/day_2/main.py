from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=2)
data = puzzle.input_data

codes = list(map(int, data.split(',')))


def run_intcode(code, i1=12, i2=2):
    i = 0
    code[1] = i1
    code[2] = i2
    while True:
        opcode = code[i]
        if opcode == 99:
            break
        elif opcode == 1:
            pos_a, pos_b, pos_r = code[i + 1], code[i + 2], code[i + 3]
            code[pos_r] = code[pos_a] + code[pos_b]
        elif opcode == 2:
            pos_a, pos_b, pos_r = code[i + 1], code[i + 2], code[i + 3]
            code[pos_r] = code[pos_a] * code[pos_b]
        else:
            print('err')
            exit(1)
        i += 4
    return code[0]


a = run_intcode(list([c for c in codes]))

print(a)
puzzle.answer_a = a

b = 0
for x in range(100):
    for y in range(100):
        z = run_intcode(list([c for c in codes]), x, y)
        if z == 19690720:
            b = 100 * x + y
            break

print(b)
puzzle.answer_b = b

