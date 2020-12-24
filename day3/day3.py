def count(dx, dy):
    res, x, y = 0, 0, 0
    while y < len(grid):
        res += grid[y][x] == '#'
        y += dy
        x = (x + dx) % len(grid[0])
    return res

grid = [x.rstrip() for x in open("day3.txt")]
print(count(3, 1))
print(count(1, 1) * count(3, 1) * count(5, 1) * count(7, 1) * count(1, 2))
