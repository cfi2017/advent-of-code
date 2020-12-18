import time

from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=18)
data = puzzle.input_data

sample1 = """1 + 2 * 3 + 4 * 5 + 6"""
sample2 = """1 + (2 * 3) + (4 * (5 + 6))"""


def find_subexpression(expr):
    i = 0
    level = 1
    while level > 0:
        if expr[i] == ")":
            level -= 1
        if expr[i] == "(":
            level += 1
        i += 1
    return i + 1


def add(x, y): return x + y


def mul(x, y): return x * y


def build_expr_tree(expr):
    i = 0
    tree = []
    while i < len(expr):
        char = expr[i]
        if char == "(":
            skip = find_subexpression(expr[i+1:])
            tree.append(build_expr_tree(expr[i + 1:skip + i - 1]))
            i += skip
            continue
        elif char == " ":
            i += 1
            continue
        elif char == "+":
            tree.append(add)
        elif char == "*":
            tree.append(mul)
        else:
            tree.append(int(char))
        i += 1
    return tree


def eval_flat_tree(tree, ops):
    nt = []
    i = 0
    while i < len(tree):
        if tree[i] in ops:
            nt[-1] = tree[i](nt[-1], tree[i+1])
            i += 1
        else:
            nt.append(tree[i])
        i += 1
    return nt


def eval_expr_tree(tree, precedence=True):
    for i in range(len(tree)):
        if isinstance(tree[i], list):
            tree[i] = eval_expr_tree(tree[i], precedence)
    if precedence:
        tree = eval_flat_tree(tree, [add])
        return eval_flat_tree(tree, [mul])[0]
    else:
        return eval_flat_tree(tree, [add, mul])[0]


res1 = 0
res2 = 0
for line in data.splitlines():
    res1 += eval_expr_tree(build_expr_tree(line), False)
    res2 += eval_expr_tree(build_expr_tree(line))


assert eval_expr_tree(build_expr_tree(sample1), False) == 71
assert eval_expr_tree(build_expr_tree(sample2), False) == 51
assert eval_expr_tree(build_expr_tree("2 * 3 + (4 * 5)"), False) == 26
assert eval_expr_tree(build_expr_tree("5 + (8 * 3 + 9 + 3 * 4 * 3)"), False) == 437
assert eval_expr_tree(build_expr_tree("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), False) == 12240
assert eval_expr_tree(build_expr_tree("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), False) == 13632

assert eval_expr_tree(build_expr_tree("1 + 2 * 3 + 4 * 5 + 6")) == 231
assert eval_expr_tree(build_expr_tree("1 + (2 * 3) + (4 * (5 + 6))")) == 51
assert eval_expr_tree(build_expr_tree("2 * 3 + (4 * 5)")) == 46
assert eval_expr_tree(build_expr_tree("5 + (8 * 3 + 9 + 3 * 4 * 3)")) == 1445
assert eval_expr_tree(build_expr_tree("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")) == 669060
assert eval_expr_tree(build_expr_tree("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")) == 23340

assert res1 == 6640667297513
assert res2 == 451589894841552
print(res1)
print(res2)


res1 = 0
res2 = 0
time_total = 0
test_count = 100
for temp_step in range(test_count):
    time_before = time.time()
    for line in data.splitlines():
        res1 += eval_expr_tree(build_expr_tree(line), False)
        res2 += eval_expr_tree(build_expr_tree(line))
    time_total += time.time() - time_before
print(test_count, "trials took", time_total / test_count)
