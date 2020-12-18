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
            tree[i] = eval_expr_tree(tree[i])
    if precedence:
        tree = eval_flat_tree(tree, [add])
        return eval_flat_tree(tree, [mul])[0]
    else:
        return eval_flat_tree(tree, [add, mul])[0]


sample1 = eval_expr_tree(build_expr_tree(sample1), False)
assert sample1 == 71
sample2 = eval_expr_tree(build_expr_tree(sample2), False)
assert sample2 == 51

res = 0
for line in data.splitlines():
    res += eval_expr_tree(build_expr_tree(line))

print(res)
