f = open('data/2.txt', 'r')
lines = f.read().splitlines()
f.close()

valid = 0
valid2 = 0
for line in lines:
    [bounds, charstr, passwd] = line.split(' ')
    [a, b] = map(int, bounds.split('-'))
    targetchar = charstr[0]
    count = 0
    for char in passwd:
        if char == targetchar:
            count += 1

    if passwd[a-1] != passwd[b-1] and (passwd[a-1] == targetchar or passwd[b-1] == targetchar):
        valid2 += 1
    if a <= count <= b:
        # valid
        valid += 1

print(valid)
print(valid2)