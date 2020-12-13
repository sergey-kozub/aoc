lines = list(open('day13.txt'))
N = int(lines[0])
buses = [int(nr) if nr != 'x' else None for nr in lines[1].split(',')]

wait = {nr - N % nr: nr for nr in buses if nr is not None}
key = min(wait)
print(key * wait[key])

lookup = [(v, k + 1) for k, v in enumerate(buses) if v is not None]
verify = lambda a, n: all(k - n % k == v % k for k, v in a)
start, step = 0, 1
for idx in range(len(lookup)):
    while not verify(lookup[:idx+1], start):
        start += step
    step *= lookup[idx][0]
print(start + 1)
