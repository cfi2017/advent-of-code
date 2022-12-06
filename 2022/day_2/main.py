from aocd.models import Puzzle
puzzle = Puzzle(year=2022, day=2)
data = puzzle.input_data

map = {
    'A': 1, # rock -> beats 3 (1 -> 3)       r 1 and l 3
    'B': 2, # paper -> beats 1 (2 -> 1)      r 2 and l 1
    'C': 3, # scissors -> beats 2 (3 -> 2)   r 3 and l 2
    'X': 1,
    'Y': 2,
    'Z': 3,
}
score_a = 0
score_b = 0
for line in data.splitlines():
    if line == "":
        continue
    [left, right] = line.split(' ')
    left = map[left]
    right = map[right]
    if left == right:
        score_a += 3
    elif (left == 3 and right == 1) or (left == 2 and right == 3) or (left == 1 and right == 2):
        score_a += 6
    score_a += right

    if right == 1:
        # lose
        if left == 1:
            score_b += 3
        elif left == 2:
            score_b += 1
        else:
            score_b += 2
    elif right == 2:
        # draw
        score_b += 3
        score_b += left
    else:
        # win
        score_b += 6
        if left == 1:
            score_b += 2
        elif left == 2:
            score_b += 3
        else:
            score_b += 1

print(score_a)
print(score_b)
puzzle.answer_a = score_a
puzzle.answer_b = score_b
