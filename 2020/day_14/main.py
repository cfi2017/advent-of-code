from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=14)
data = puzzle.input_data

# data = """mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
# mem[8] = 11
# mem[7] = 101
# mem[8] = 0"""


def calc_addresses(addr, mask):
    addresses = []
    b = ('{0:0%db}' % len(mask)).format(addr)
    for i in range(len(mask)):
        if mask[i] != '0':
            b = b[:i] + mask[i] + b[i + 1:]
    left = [b]
    while len(left) > 0:
        addr = left.pop(0)
        next_x = addr.find('X')
        if next_x == -1:
            addresses.append(int(addr, 2))
        else:
            left.append(addr[:next_x] + '1' + addr[next_x+1:])
            left.append(addr[:next_x] + '0' + addr[next_x+1:])
    return addresses


mem = {}
memv2 = {}
mask = ''
ones = 0
zeros = 0
floating_its = 0
for line in data.splitlines():
    if 'mask' in line:
        mask = line.split(' ')[2]
        ones = int(mask.replace('X', '0'), 2)  # run or
        zeros = int(mask.replace('X', '1'), 2)  # run and
        floating_its = 2 ** mask.count('X')
    else:
        val = line.split(' ')
        addr, val = int(val[0][4:-1]), int(val[2])
        valv1 = val | ones
        valv1 = valv1 & zeros
        mem[addr] = valv1

        # part 2
        addresses = calc_addresses(addr, mask)
        for ad in addresses:
            memv2[ad] = int(val)


# 1010001011010001010
# 000000000000000001010001011010001010 value
# 010100010000101011010000000101010001 ones
# 010111011001111011111000000101110001 zeroes
# 010100010000101011010000000101010001 # new value

s = sum(list(mem.values()))
print(s)
puzzle.answer_a = s

s = sum(list(memv2.values()))
print(s)
puzzle.answer_b = s


