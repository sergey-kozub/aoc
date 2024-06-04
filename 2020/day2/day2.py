import re

input_ = [re.match(r'(\d+)-(\d+) (\w): (\w+)', line).groups()
          for line in open("day2.txt")]
print(sum(1 for a, b, c, s in input_ if int(a) <= len(re.findall(c, s)) <= int(b)))
print(sum(1 for a, b, c, s in input_ if (s[int(a)-1] == c) + (s[int(b)-1] == c) == 1))
