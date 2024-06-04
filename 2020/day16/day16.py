import re

# parse input
content = open('day16.txt').read().rstrip().split('\n\n')
rules = {m[0]: set(v for i in range(1, 5, 2) for v in range(int(m[i]), int(m[i+1]) + 1))
         for m in re.findall(r'^(.+): (\d+)-(\d+) or (\d+)-(\d+)', content[0], re.M)}
your = list(map(int, content[1].split('\n')[1].split(',')))
nearby = [list(map(int, line.split(','))) for line in content[2].split('\n')[1:]]

# detect invalid
values = set(v for s in rules.values() for v in s)
valid = [t for t in nearby if all(v in values for v in t)]
print(sum(v for t in nearby for v in t if v not in values))

# resolve mapping
options = [[k for k, v in rules.items() if all(x in v for x in a)] for a in zip(*valid)]
ordered = sorted(enumerate(options), key=lambda x: len(x[1]))
names = {}
for k, v in ordered:
    names[[s for s in v if s not in names][0]] = k

# calculate result
result = 1
for k, v in names.items():
    if k.startswith('departure'):
        result *= your[v]
print(result)
