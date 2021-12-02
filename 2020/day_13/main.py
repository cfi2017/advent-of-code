from aocd.models import Puzzle
from functools import reduce
from sympy.ntheory.modular import crt
puzzle = Puzzle(year=2020, day=13)
data = puzzle.input_data

# data = """939
# 7,13,x,x,59,x,31,19"""


def chinese_remainder(n, a):
    sum=0
    prod=reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n,a):
        p=prod/n_i
        sum += a_i* mul_inv(p, n_i)*p
    return sum % prod


def mul_inv(a, b):
    b0= b
    x0, x1= 0,1
    if b== 1: return 1
    while a>1 :
        q=a// b
        a, b= b, a%b
        x0, x1=x1 -q *x0, x0
    if x1<0 : x1+= b0
    return x1


[min_depart, busses] = data.splitlines()
busses = list(filter(lambda x: x != 'x', busses.split(',')))
busses = list(map(int, busses))
min_depart = int(min_depart)

[min_wait, id] = -1, 0
for bus in busses:
    wait = bus - (min_depart % bus)
    if wait < min_wait or min_wait == -1:
        min_wait = wait
        id = bus

print(min_wait, id)
# puzzle.answer_a = min_wait * id

n = []
a = []
busses = data.splitlines()[1].split(',')
for i, b in enumerate(busses):
    if b != 'x':
        n.append(int(b))
        a.append(int(b) - i)

res = int(chinese_remainder(n, a))
print(res)
print(crt(n, a))
puzzle.answer_b = crt(n, a)[0]
