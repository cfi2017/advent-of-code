from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day=16)
data = puzzle.input_data


def check_rules(rules, x):
    succeeded = []
    for r in rules:
        if r[0][0] <= x <= r[0][1] or r[1][0] <= x <= r[1][1]:
            succeeded.append(r)
    return succeeded


def check_validity(rules, ticket):
    failed = []
    for x in ticket:
        if len(check_rules(rules, x)) == 0:
            failed.append(x)
    return failed


[rules, my_ticket, nearby_tickets] = data.split('\n\n')
rules = rules.splitlines()
rules = list(map(lambda x: x.split(': ')[1].split(' '), rules))
rules = list(map(lambda x: (list(map(int, x[0].split('-'))), list(map(int, x[2].split('-')))), rules))
my_ticket = list(map(int, my_ticket.splitlines()[1].split(',')))
nearby_tickets = list(map(lambda x: list(map(int, x.split(','))), nearby_tickets.splitlines()[1:]))

valid = []
invalid = 0
for ticket in nearby_tickets:
    add = sum(check_validity(rules, ticket))
    invalid += add
    if add == 0:
        valid.append(ticket)
print(invalid)
# puzzle.answer_a = invalid

candidate_dict = {}
for i, r in enumerate(rules):
    candidates = [i for i, x in enumerate(valid[0]) if r[0][0] <= x <= r[0][1] or r[1][0] <= x <= r[1][1]]
    for ticket in valid[1:]:
        candidates = [i for i in candidates if r[0][0] <= ticket[i] <= r[0][1] or r[1][0] <= ticket[i] <= r[1][1]]
        if len(candidates) == 1:
            break
    candidate_dict[i] = candidates

positions = {}
while len(candidate_dict) > 0:
    done = []
    added_rules = []
    for rule in candidate_dict:
        if len(candidate_dict[rule]) == 1:
            positions[rule] = candidate_dict[rule][0]
            done.append(positions[rule])
            added_rules.append(rule)
    for added in added_rules:
        del candidate_dict[added]
    for rule in candidate_dict:
        candidate_dict[rule] = [x for x in candidate_dict[rule] if x not in done]

print(positions)
res = 1
for i in range(6):
    res *= my_ticket[positions[i]]
print(res)
puzzle.answer_b = res
