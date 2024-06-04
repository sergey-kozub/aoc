from collections import Counter

a = sorted(map(int, open('day10.txt')))
c = Counter([a[i] - (a[i - 1] if i else 0) for i in range(len(a))] + [3])
print(c[1] * c[3])

b, m = list(reversed(a)) + [0], [1]
for i in range(len(a) + 1):
    s = m[-1]
    if i > 1 and b[i-2] - b[i] <= 3: s += m[-2]
    if i > 2 and b[i-3] - b[i] <= 3: s += m[-3]
    m.append(s)
print(m[-1])
