input_ = "137826495"

class Node:
    def __init__(self, label):
        self.label = label
        self.next = None

class Circle:
    def __init__(self, initial, total):
        self.nodes = [Node(i + 1) for i in range(total)]
        m, a = len(initial), [int(c) - 1 for c in initial]
        for i, n in enumerate(map(int, initial)):
            p = a[i + 1] if i < m - 1 else (m if m < total else a[0])
            self.nodes[n - 1].next = self.nodes[p]
        for i in range(m, total):
            p = i + 1 if i < total - 1 else a[0]
            self.nodes[i].next = self.nodes[p]
        self.total = total

    def get(self, label, count):
        node, res = self.nodes[label - 1], []
        for _ in range(count):
            node = node.next
            res.append(node.label)
        return res

    def move(self, label, count, block=3):
        node = self.nodes[label - 1]
        for _ in range(count):
            temp, res = node, []
            for _ in range(block + 1):
                temp = temp.next
                res.append(temp)

            labels = [x.label for x in res[:-1]]
            n = node.label
            while True:
                n -= 1
                if not n: n = self.total
                if n not in labels: break
            dest = self.nodes[n - 1]

            res[-2].next = dest.next
            dest.next = res[0]
            node.next = res[-1]
            node = node.next

small = Circle(input_, 9)
small.move(1, 100)
print(''.join(map(str, small.get(1, 8))))

big = Circle(input_, 1000000)
big.move(1, 10000000)
a, b = big.get(1, 2)
print(a * b)
