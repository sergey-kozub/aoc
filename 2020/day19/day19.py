input_ = open("day19.txt").read()
all_rules, all_messages = input_.split('\n\n')
rules = {int(k): v[1] if v[0] == '"' else
         [tuple(map(int, r.split(' '))) for r in v.split(' | ')]
         for k, v in [s.split(': ') for s in all_rules.split('\n')]}
messages = all_messages.rstrip().split('\n')

# Part 1
options = {k: {v} for k, v in rules.items() if isinstance(v, str)}
def build(key):
    if key in options:
        return options[key]
    values = set()
    for seq in rules[key]:
        left = ['']
        for right in map(build, seq):
            left = [l + r for l in left for r in right]
        values.update(left)
    options[key] = values
    return values

build(0)
print(sum(s in options[0] for s in messages))

# Part 2
l42 = len(next(iter(options[42])))
l31 = len(next(iter(options[31])))

def matches_8(msg):
    return any(msg.startswith(p) for p in options[42]) and \
        (len(msg) == l42 or matches_8(msg[l42:]))

def matches_11(msg):
    return any(msg.startswith(p) for p in options[42]) and \
        any(msg.endswith(s) for s in options[31]) and \
        (len(msg) == l42 + l31 or matches_11(msg[l42:-l31]))

def matches_0(msg):
    return any(matches_8(msg[:i]) and matches_11(msg[i:])
               for i in range(0, len(msg)))

print(sum(matches_0(s) for s in messages))
