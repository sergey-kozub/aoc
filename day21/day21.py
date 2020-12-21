import functools
import re

input_ = [(set(m[1].split(' ')), set(m[2].split(', ')))
          for m in [re.match(r'(.+) \(contains (.+)\)', line)
          for line in open("day21.txt")]]

allergens = {}
unknown = set(s for x in input_ for s in x[1])
while unknown:
    for test in unknown:
        l, r = [functools.reduce(lambda a, b: a & b, t)
                for t in zip(*[x for x in input_ if test in x[1]])]
        l -= set(allergens.keys())
        r -= set(allergens.values())
        if len(l) == len(r) and r:
            allergens.update(zip(l, r))
            unknown -= r
            break

clean = lambda s: s - set(allergens.keys())
print(functools.reduce(lambda c, x: c + len(clean(x[0])), input_, 0))
print(','.join(x[0] for x in sorted(allergens.items(), key=lambda x: x[1])))
