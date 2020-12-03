f = open('data/1.txt', 'r')
lines = f.read().splitlines()
f.close()

for i, a in enumerate(lines):
    for b in lines[i:]:
        if int(a) + int(b) == 2020:
            print(int(a) * int(b))

for i, a in enumerate(lines):
    for i2, b in enumerate(lines[i:]):
        for c in lines[i2:]:
            if int(a) + int(b) + int(c) == 2020:
                print(int(a) * int(b) * int(c))
