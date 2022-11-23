from aocd.models import Puzzle
from aocd import submit
from functools import reduce

puzzle = Puzzle(year=2021, day=8)
data = puzzle.input_data
data = """be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"""
lines = data.splitlines()

result_a = 0
result_b = 0

lines = list(map(lambda x: x.split(' | '), lines))
inputs = list(map(lambda x: x[1].split(' '), lines))
print(inputs)
inputs = reduce(list.__add__, inputs)
print(inputs)
total_unique = 0
for i in inputs:
    if len(i) in [2, 4, 3, 7]:
        total_unique += 1
print(total_unique)

submit(total_unique, part="a", year=2021, day=8)

def intersect(candidates, chars, values):
    for c in chars:
        if c not in candidates:
            candidates[c] = values
        candidates[c] = [cc for cc in candidates[c] if cc in values]
    return candidates

def solve(values: list, output: list) -> int:
    values.sort(key=len)
    candidates = {}
    for value in values:
        if len(value) == 2:
            candidates = intersect(candidates, 'cf', values)
        elif len(value) == 3:
            candidates = intersect(candidates, 'cfa', values)
        elif len(value) == 4:
            candidates = intersect(candidates, 'cfbd', values)
    print(candidates)
    return 0

s = 0
for line in lines:
    v, o = line[0], line[1]
    s += solve(v.split(' '), o.split(' '))
print(s)

# submit(result_b, part="b", year=2021, day=)

