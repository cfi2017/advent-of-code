from aocd.models import Puzzle
from aocd import submit
from collections import deque
from numpy import zeros

puzzle = Puzzle(year=2021, day=6)
data = puzzle.input_data
# data = '3,4,3,1,2'

result_a = 0
result_b = 0

nums_arr = list(map(int, data.split(',')))
nums = deque(zeros(9))
for num in nums_arr:
    nums[num] += 1
for i in range(256):
    nums.rotate(-1)
    nums[6] += nums[8]
    if i == 79:
        result_a = int(sum(nums))
result_b = int(sum(nums))

submit(result_a, part="a", year=2021, day=6)
submit(result_b, part="b", year=2021, day=6)

