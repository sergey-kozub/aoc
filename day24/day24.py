moves = {
    'e': (1, 0),
    'w': (-1, 0),
    'ne': (0, 1),
    'nw': (-1, 1),
    'se': (1, -1),
    'sw': (0, -1),
}

def parse(line):
    res = [(0, 0)]
    while line:
        key = line[0] if line[0] in moves else line[:2]
        dx, dy = moves[key]
        res.append((res[-1][0] + dx, res[-1][1] + dy))
        line = line[len(key):]
    return res[-1]

def move(tiles):
    res, adj = tiles.copy(), set()
    for b in tiles:
        a = [(b[0] + dx, b[1] + dy) for dx, dy in moves.values()]
        n = sum(x in tiles for x in a)
        if n == 0 or n > 2: res.remove(b)
        adj.update(a)
    for w in adj:
        a = [(w[0] + dx, w[1] + dy) for dx, dy in moves.values()]
        n = sum(x in tiles for x in a)
        if n == 2: res.add(w)
    return res

input_ = [x.rstrip() for x in open('day24.txt')]
tiles = set()
for line in input_:
    pos = parse(line)
    if pos not in tiles:
        tiles.add(pos)
    else:
        tiles.remove(pos)
print(len(tiles))

for _ in range(100):
    tiles = move(tiles)
print(len(tiles))
