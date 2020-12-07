import re

rules = {}
for line in open('day7.txt'):
    m = re.findall(r'(\d*)\s*(\w+ \w+) bags?', line)
    rules[m[0][1]] = {s: int(n) for n, s in m[1:] if s != 'no other'}

parents = {}
for src, bags in rules.items():
    for dst in bags.keys():
        parents.setdefault(dst, set()).add(src)

def all_parents(bag):
    res = parents.get(bag, set())
    for key in list(res): res.update(all_parents(key))
    return res
print(len(all_parents('shiny gold')))

def count_all(bag):
    return sum(v * count_all(k) for k, v in rules.get(bag, {}).items()) + 1
print(count_all('shiny gold') - 1)
