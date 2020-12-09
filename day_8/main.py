from aocd.models import Puzzle

puzzle = Puzzle(year=2020, day=8)
input = puzzle.input_data

ran = []

# input = """nop +0
# acc +1
# jmp +4
# acc +3
# jmp -3
# acc -99
# acc +1
# nop -4
# acc +6"""

input = list(map(lambda x: x.split(' '), input.splitlines()))
input = list(map(lambda x: (x[0], int(x[1])), input))


# returns True if we found the right path, false otherwise
def recurse(instructions, pos=0, acc=0, stack=None, branched=False, i=0):
    print(i)
    # run instruction
    if stack is None:
        stack = []
    if pos >= len(instructions):
        print('hit exit condition')
        return acc
    stack.append(pos)
    op, ins = instructions[pos]
    if op == "acc":
        acc += ins
        pos += 1
    elif op == "jmp":
        pos += ins
    elif op == "nop":
        pos += 1
    # loop detection
    if pos in stack:
        stack.pop()
        return None
    # recurse and check recurse output
    r = recurse(instructions, pos, acc, stack, branched, i + 1)
    if r is None:
        if branched:
            print('bt: already branched')
            return None
        else:
            print('trying to branch')
            # change current condition, branched = true from here on out
            if op == "acc":
                print('bt: cannot branch acc')
                return None
            elif op == "jmp":
                print('bt: negating jmp')
                pos -= ins  # negate previous jmp (same as nop)
            elif op == "nop":
                print('bt: treating nop as jmp')
                pos += ins  # treat nop as jmp
            return recurse(instructions, pos, acc, stack, True, i + 1)
    else:
        return r


a = recurse(input)
print(a)
# puzzle.answer_a = acc
