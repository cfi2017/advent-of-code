from aocd.models import Puzzle
import re
puzzle = Puzzle(year=2020, day=4)
input = puzzle.input_data
valid = 0

fields = [
    'byr',
    'iyr',
    'eyr',
    'hgt',
    'hcl',
    'ecl',
    'pid',
    'cid'
]


def validateHgt(v):
    if re.fullmatch(r"[0-9]+(cm|in)", v) is not None:
        if 'cm' in v:
            return 150 <= int(v[:-2]) <= 193
        else:
            return 59 <= int(v[:-2]) <= 76


validators = {
    'byr': lambda x: re.fullmatch(r"[0-9]{4}", x) is not None and 1920 <= int(x) <= 2002,
    'iyr': lambda x: re.fullmatch(r"[0-9]{4}", x) is not None and 2010 <= int(x) <= 2020,
    'eyr': lambda x: re.fullmatch(r"[0-9]{4}", x) is not None and 2020 <= int(x) <= 2030,
    'hgt': validateHgt,
    'hcl': lambda x: re.fullmatch(r"#[0-9a-f]{6}", x) is not None,
    'ecl': lambda x: re.fullmatch(r"(amb|blu|brn|gry|grn|hzl|oth)", x) is not None,
    'pid': lambda x: re.fullmatch(r"[0-9]{9}", x) is not None,
    'cid': lambda x: True
}

def parse(passport):
    elements = passport.split(' ')
    elements = map(lambda x: x.split('\n'), elements)
    sublist = []
    for list in elements:
        for element in list:
            sublist.append(element)
    return sublist


def validate(passport):
    required = fields.copy()
    required.remove('cid')
    print(passport)
    if not all(t in passport for t in required):
        return False
    if all(validators[t](value) for [t, value] in map(lambda x: x.split(':'), passport)):
        return True


passports = input.split('\n\n')


passports = list(map(parse, passports))
passports = list(map(validate, passports)).count(True)
print(passports)
