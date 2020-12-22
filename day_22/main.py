import time
from copy import deepcopy, copy
import numpy as np

from aocd.models import Puzzle

puzzle = Puzzle(year=2020, day=22)
data = puzzle.input_data
data = open('mingmingrr.txt').read()

p1, p2 = data.split('\n\n')
p1 = list(map(int, p1.splitlines()[1:]))
p2 = list(map(int, p2.splitlines()[1:]))

print(p1, p2)


def is_infinite(deck, rounds):
    d = tuple(deck)
    if d in rounds:
        return True
    rounds.add(d)
    return False


def score(deck1, deck2):
    deck = deck1 + deck2
    score = 0
    for i in range(len(deck)):
        score += deck[::-1][i] * (i + 1)
    return score


def rec(p1, p2):
    prevs1 = set()
    prevs2 = set()
    while True:
        if is_infinite(p1, prevs1) or is_infinite(p2, prevs2):
            return True
        if len(p1) == 0:
            return False
        if len(p2) == 0:
            return True

        n1, n2 = p1.pop(0), p2.pop(0)
        if len(p1) < n1 or len(p2) < n2:
            if n1 > n2:
                p1.append(n1)
                p1.append(n2)
            else:
                p2.append(n2)
                p2.append(n1)
        else:
            if rec(copy(p1[:n1]), copy(p2[:n2])):
                p1.append(n1)
                p1.append(n2)
            else:
                p2.append(n2)
                p2.append(n1)


original = copy(p1), copy(p2)

while True:
    n1, n2 = p1.pop(0), p2.pop(0)
    if n1 > n2:
        p1.append(n1)
        p1.append(n2)
    elif n1 < n2:
        p2.append(n2)
        p2.append(n1)
    else:
        print('tie')

    if len(p1) == 0 or len(p2) == 0:
        break


time_total = 0
test_count = 10
for temp_step in range(test_count):
    cache = {}
    time_before = time.time()
    p1, p2 = copy(original[0]), copy(original[1])
    rec(p1, p2)
    sum = score(p1, p2)
    time_total += time.time() - time_before
print(test_count, "trials averaged", time_total / test_count)

p1, p2 = copy(original[0]), copy(original[1])
rec(p1, p2)
sum = score(p1, p2)
print(sum)


