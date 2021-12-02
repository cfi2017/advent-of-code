import time
import asyncio

from aocd.models import Puzzle
import re
import sys
puzzle = Puzzle(year=2020, day=19)
data = puzzle.input_data

[rules, data] = data.split('\n\n')
rules = rules.splitlines()
data = data.splitlines()

cache = {}


async def asyncmatch(msg, r):
    return bool(r.fullmatch(msg))


def resolve(n):
    if n in cache:
        return cache[n]
    if n == '8':
        cache[n] = resolve('42') + '+'
        return cache[n]
    elif n == '11':
        n42 = resolve('42')
        n31 = resolve('31')
        # just brute force the first 1000 options
        rule = '(?:' + '|'.join(f'{n42}{{{n}}}{n31}{{{n}}}' for n in range(1, 100)) + ')'
        cache[n] = rule
        return rule
    rule = rules[n]
    if rule[0] == '"':
        cache[n] = rule[1]
        return rule[1]
    else:
        parts = rule.split(' | ')
        tokens = []
        for part in parts:
            vals = part.split(' ')
            tokens.append(''.join(resolve(x) for x in vals))

    rule = '(?:' + '|'.join(tokens) + ')'
    cache[n] = rule
    return rule


async def part2():
    r = resolve('0')
    r = re.compile(r)
    funcs = [asyncmatch(line, r) for line in data]
    return (await asyncio.gather(**funcs)).count(True)


rules = dict([rule.split(': ', 1) for rule in rules])

time_total = 0
test_count = 100
count = 0
for temp_step in range(test_count):
    cache = {}
    time_before = time.time()
    count = asyncio.run(part2())
    time_total += time.time() - time_before
print(test_count, "trials took", time_total / test_count)


print(count)
# puzzle.answer_b = count
