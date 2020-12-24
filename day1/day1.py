S = set(map(int, open("day1.txt")))
r1 = [(a, 2020 - a) for a in S if (2020 - a) in S][0]
r2 = [(a, b, 2020 - a - b) for a in S for b in S
      if a != b and (2020 - a - b) in S][0]
print(r1[0] * r1[1])
print(r2[0] * r2[1] * r2[2])
