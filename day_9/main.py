from aocd.models import Puzzle
import time
puzzle = Puzzle(year=2020, day=9)
lines = puzzle.input_data

lines = lines.splitlines()
lines = list(map(int, lines))


def part_1(nums, pre=25):
    i = pre
    while i < len(nums):
        if not check_last(nums[i-pre:i], nums[i]):
            # puzzle.answer_a = input[i]
            return nums[i]
        i = i + 1


def check_last(nums, num):
    for i, x in enumerate(nums):
        for y in nums[i:]:
            if x != y and x + y == num:
                return True
    return False


def part_2(nums, val):
    for i, x in enumerate(nums):
        s = nums[i]
        for i2, y in enumerate(nums[i + 1:]):
            s += y
            if s == val:
                r = nums[i:i + 1 + i2]
                return min(r) + max(r)
            elif s > val:
                break


answer_a = part_1(lines)
answer_b = part_2(lines, answer_a)
print(answer_a, answer_b)

time_total = 0
test_count = 1000
for temp_step in range(test_count):
    time_before = time.time()
    part_1(lines)
    time_total += time.time() - time_before
print(test_count, "trials of part 1 took", time_total / test_count)
time_total = 0
for temp_step in range(test_count):
    time_before = time.time()
    part_2(lines, answer_a)
    time_total += time.time() - time_before
print(test_count, "trials of part 2 took", time_total / test_count)
time_total = 0
for temp_step in range(test_count):
    time_before = time.time()
    part_1(lines)
    part_2(lines, answer_a)
    time_total += time.time() - time_before
print(test_count, "trials of both parts took", time_total / test_count)
