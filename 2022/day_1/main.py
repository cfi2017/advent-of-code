from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=1)
data = puzzle.input_data

elves = data.split('\n\n')
calorie_list = []
for elf in elves:
    calories = sum(map(int, elf.splitlines()))
    calorie_list.append(calories)
calorie_list.sort(reverse=True)
puzzle.answer_a = calorie_list[0]
puzzle.answer_b = sum(calorie_list[:3])
