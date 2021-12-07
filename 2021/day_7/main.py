from aocd.models import Puzzle
from aocd import submit
from statistics import median


puzzle = Puzzle(year=2021, day=7)
data = puzzle.input_data
# data = '16,1,2,0,4,2,7,1,2,14'

data = list(map(int, data.split(',')))

result_a = 0
result_b = 999999999999999


print(median(data))

result_a = int(sum([abs(x - median(data)) for x in data]))
print(result_a)


for i in range(min(data), max(data)):
    current = int(sum([(abs(x - i) * (abs(x - i) + 1) // 2) for x in data]))
    if current < result_b:
        result_b = current

print(result_b)


submit(result_a, part="a", year=2021, day=7)
submit(result_b, part="b", year=2021, day=7)

