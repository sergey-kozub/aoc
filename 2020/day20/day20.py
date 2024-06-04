class Tile:
    def __init__(self, id_, grid):
        self.id_ = id_
        self.grid = grid

    def __repr__(self):
        return f"Tile {self.id_}:\n" + '\n'.join(self.grid)
    
    def __hash__(self):
        return self.id_

    def rotated(self):
        return Tile(self.id_, [''.join(reversed(x)) for x in zip(*self.grid)])

    def hflipped(self):
        return Tile(self.id_, [''.join(reversed(x)) for x in self.grid])

    def vflipped(self):
        return Tile(self.id_, list(reversed(self.grid)))

    def option(self, n):
        if n == 0: return self
        if n == 1: return self.rotated()
        if n == 2: return self.hflipped()
        if n == 3: return self.vflipped()
        if n == 4: return self.rotated().rotated()
        if n == 5: return self.rotated().hflipped()
        if n == 6: return self.rotated().vflipped()
        if n == 7: return self.rotated().rotated().rotated()

    def match_top(self, other):
        return self.grid[0] == other.grid[-1]

    def match_right(self, other):
        return all(v[-1] == other.grid[k][0] for k, v in enumerate(self.grid))

    def match_bottom(self, other):
        return self.grid[-1] == other.grid[0]

    def match_left(self, other):
        return all(v[0] == other.grid[k][-1] for k, v in enumerate(self.grid))

    def find_all(self, pattern):
        count = 0
        pos = [(x, y) for y, line in enumerate(pattern)
               for x, ch in enumerate(line) if ch == '#']
        for y in range(len(self.grid) - len(pattern) + 1):
            for x in range(len(self.grid[0]) - len(pattern[0]) + 1):
                count += all(self.grid[y + dy][x + dx] == '#' for dx, dy in pos)
        return count

input_ = open("day20.txt").read().rstrip()
tiles = [Tile(int(lines[0].split(' ')[1].rstrip(':')), lines[1:])
    for lines in [block.split('\n') for block in input_.split('\n\n')]]

# Part 1
pos = {(0, 0): tiles[0]}
unassigned = set(tiles[1:])
while unassigned:
    match = None
    for tile in unassigned:
        for temp in [tile.option(i) for i in range(8)]:
            for (x, y), v in pos.items():
                if temp.match_top(v): match = temp, x, y - 1
                if temp.match_right(v): match = temp, x - 1, y
                if temp.match_bottom(v): match = temp, x, y + 1
                if temp.match_left(v): match = temp, x + 1, y
        if match: break
    pos[match[1:]] = match[0]
    unassigned.remove(tile)

min_, max_ = min(pos), max(pos)
res, c1, c2 = 1, (min_[0], max_[1]), (max_[0], min_[1])
for p in (min_, max_, c1, c2): res *= pos[p].id_
print(res)

# Part 2
sea = Tile(0, [''.join(pos[(tx, ty)].grid[py][px]
    for tx in range(min_[0], max_[0] + 1) for px in range(1, 9))
    for ty in range(max_[1], min_[1] - 1, -1) for py in range(1, 9)])
monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
""".strip('\n').split('\n')
count = lambda a: sum(c == '#' for s in a for c in s)
n_monsters = max(sea.option(i).find_all(monster) for i in range(8))
print("Monsters:", n_monsters)
print(count(sea.grid) - n_monsters * count(monster))
