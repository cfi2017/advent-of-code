from functools import reduce


def calc_slope(incx, incy):
    x = 0
    y = 0
    count = 0
    while y < len(grid) - 1:
        y = y + incy
        x = x + incx
        if x >= len(grid[0]):
            x = x % len(grid[0])
        if grid[y][x] == "#":
            count = count + 1
    return count


def calc_slope_arr(arr):
    return calc_slope(arr[0], arr[1])


f = open('input.txt', 'r')
grid = f.read().splitlines()
f.close()

print(calc_slope(3, 1))

print('part two')

slopes = [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2]
]
result = reduce(lambda x, y: x * y, map(calc_slope_arr, slopes))
print(result)
