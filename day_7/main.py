from aocd.models import Puzzle
import time

puzzle = Puzzle(year=2020, day=7)
input = puzzle.input_data


# recursively finds the total bags contained in a bag (including the containing bag)
def count_inner_bags(rules, color):
    child_counts = (rules[color][child] * (count_inner_bags(rules, child) + 1) for child in rules[color])
    return sum(child_counts)


# parse takes a string line and returns a touple (color, {'child_color': count})
def parse(line):
    tokens = line.split(' ')
    color = ' '.join(tokens[:2])
    if ' no ' in line:
        return color, {}
    child_count = line.count(',') + 1
    children = {' '.join(tokens[5 + i * 4:5 + i * 4 + 2]): int(tokens[4 + i * 4]) for i in range(child_count)}
    return color, children


macguffin = 'shiny gold'
rules = dict(map(parse, input.splitlines()))


# calc part 1
def part_1():
    # variables for part 1
    containing_bags = [macguffin]
    last_length = -1
    while len(containing_bags) > last_length:
        last_length = len(containing_bags)
        containing_bags = {*containing_bags, *[bag for (bag, children) in rules.items()
                                               if any(colour in children for colour in containing_bags[last_length:])]}
    return len(containing_bags)


# print both parts
print(part_1() - 1, count_inner_bags(rules, macguffin))

time_total = 0
test_count = 100
for temp_step in range(test_count):
    time_before = time.time()
    part_1()
    time_total += time.time() - time_before
print(test_count, "trials took", time_total / test_count)
time_total = 0
for temp_step in range(test_count):
    time_before = time.time()
    count_inner_bags(rules, macguffin)
    time_total += time.time() - time_before
print(test_count, "trials took", time_total / test_count)
# 2.9e-05 => 0.000029

