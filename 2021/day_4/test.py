winning_numbers = {}
from aocd.models import Puzzle
from aocd import submit
from functools import reduce

puzzle = Puzzle(year=2021, day=4)
data = puzzle.input_data
def win_found(board):
    # Check rows & columns.
    rows, cols = {}, {}
    i = 0
    for row in board:
        j = 0
        for num in row:
            if j not in cols:
                cols[j] = True
            cols[j] = cols[j] and row[num]
            j += 1

            if i not in rows:
                rows[i] = True
            rows[i] = rows[i] and row[num]

        if rows[i]:
            return True

        i += 1

    for i in cols:
        if cols[i]:
            return True

    return False


def check_win(called, board, board_index):
    count = 0
    for num in called:
        for row in board:
            if num in row:
                row[num] = True
        count += 1

        if count >= 5:
            if win_found(board):
                winning_numbers[board_index] = num
                return count

    return -1

i, j = 0, 0
called, boards, win_counts = list(), list(), list()
for line in data.splitlines():
    if i == 0 and len(line) != 0:
        called = line.strip().replace('\n', '').split(",")
    elif len(line) == 0:
        if boards:
            win_counts.append(check_win(called, boards[i - 1], i - 1))
        boards.append(list())
        i += 1
        j = 0
    else:
        nums = line.split()
        boards[i - 1].append(dict())
        k = 0
        for num in nums:
            boards[i - 1][j][num] = False
            k += 1
        j += 1

result = 0
for row in boards[win_counts.index(max(win_counts))]:
    for num in row:
        if not row[num]:
            result += int(num)

print(result * int(winning_numbers[win_counts.index(max(win_counts))]))