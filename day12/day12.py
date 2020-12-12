import math

class Ship:
    def __init__(self, dir_x, dir_y, target=False):
        self.x, self.y = 0, 0
        self.wx, self.wy = dir_x, dir_y
        self.target = target

    def move(self, line):
        cmd, dist = line[0], int(line[1:])
        if cmd in 'NS':
            dy = dist * (1 if cmd == 'N' else -1)
            if self.target: self.wy += dy
            else: self.y += dy
        elif cmd in 'EW':
            dx = dist * (1 if cmd == 'E' else -1)
            if self.target: self.wx += dx
            else: self.x += dx
        elif cmd in 'LR':
            angle = dist * (-1 if cmd == 'L' else 1)
            x, y, rad = self.wx, self.wy, angle * math.pi / 180
            self.wx = x * math.cos(rad) + y * math.sin(rad)
            self.wy = -x * math.sin(rad) + y * math.cos(rad)
        elif cmd == 'F':
            self.x += self.wx * dist
            self.y += self.wy * dist

    def move_all(self, data):
        for line in data: self.move(line)
        return self.x, self.y


data = list(open('day12.txt'))
result = lambda x: round(abs(x[0]) + abs(x[1]))
print(result(Ship(1, 0, False).move_all(data)))
print(result(Ship(10, 1, True).move_all(data)))
