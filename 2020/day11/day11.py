
class Grid:
    def __init__(self, grid, near):
        self.grid = grid
        self.width = len(grid[0])
        self.height = len(grid)
        self.near = near

    def seat(self, row, col):
        if (0 <= row < self.height) and (0 <= col < self.width):
            return self.grid[row][col]

    def visible(self, row, col, dy, dx, dist):
        for i in range(1, dist + 1):
            seat = self.seat(row + dy * i, col + dx * i)
            if seat != '.': return seat

    def next_value(self, row, col):
        value = self.grid[row][col]
        if value == '.': return '.'
        visible = [self.visible(row, col, i, j, 1 if self.near else 1000)
                   for i in range(-1, 2) for j in range(-1, 2) if i or j]
        count = sum(s == '#' for s in visible)
        flip = count >= (4 if self.near else 5) if value == '#' else not count
        return value if not flip else ('L' if value == '#' else '#')

    def new_state(self):
        return [''.join(self.next_value(i, j)
                for j in range(self.width))
                for i in range(self.height)]

    def stabilize(self):
        while True:
            self.grid, prev = self.new_state(), self.grid
            if ''.join(self.grid) == ''.join(prev): return self.count

    @property
    def count(self):
        return sum(c == '#' for c in ''.join(self.grid))


seats = [s.rstrip() for s in open('day11.txt')]
count = lambda grid: sum(c == '#' for c in ''.join(grid))
print(Grid(seats, True).stabilize())
print(Grid(seats, False).stabilize())
