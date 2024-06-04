import collections

Point = collections.namedtuple('Point', ['x', 'y', 'z', 'w'])
class Grid:
    def __init__(self, ndim, initial):
        assert ndim in (3, 4)
        self.ndim = ndim
        self.points = [
            Point(x, y, 0, 0)
            for y, v in enumerate(initial)
            for x, c in enumerate(v) if c == '#']
        self.index = {v: k for k, v in enumerate(self.points)}

    def around(self, pt):
        w_delta = 1 if self.ndim == 4 else 0
        for x in range(pt.x - 1, pt.x + 2):
            for y in range(pt.y - 1, pt.y + 2):
                for z in range(pt.z - 1, pt.z + 2):
                    for w in range(pt.w - w_delta, pt.w + w_delta + 1):
                        if x != pt.x or y != pt.y or z != pt.z or w != pt.w:
                            yield Point(x, y, z, w)

    def candidates(self):
        inactive = set()
        for pt in self.points:
            for t in self.around(pt):
                if t not in self.index:
                    inactive.add(t)
        return inactive

    def update(self):
        points = []
        for pt in self.points:
            count = sum(1 for t in self.around(pt) if t in self.index)
            if count in (2, 3):
                points.append(pt)
        for pt in self.candidates():
            count = sum(1 for t in self.around(pt) if t in self.index)
            if count == 3:
                points.append(pt)
        self.points = points
        self.index = {v: k for k, v in enumerate(points)}

    def simulate(self, steps):
        for _ in range(steps): self.update()
        return len(self.index)

    def slice(self, z, w=0):
        pts = [pt for pt in self.points if pt.z == z and pt.w == w]
        y_min, y_max = min(pt.y for pt in pts), max(pt.y for pt in pts)
        x_min, x_max = min(pt.x for pt in pts), max(pt.x for pt in pts)
        result = ''
        for y in range(y_min, y_max + 1):
            for x in range(x_min, x_max + 1):
                result += '#' if Point(x, y, z, w) in self.index else '.'
            result += '\n'
        return result

    def __repr__(self):
        result = ''
        wz_all = sorted(set((pt.w, pt.z) for pt in self.points))
        for w, z in wz_all:
            if result: result += '\n'
            result += f'z={z}, w={w}\n'
            result += self.slice(z, w)
        return result

input_ = [line.rstrip() for line in open('day17.txt')]
print(Grid(3, input_).simulate(6))
print(Grid(4, input_).simulate(6))
