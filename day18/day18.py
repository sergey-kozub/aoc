
class Calc:
    ops = {'+': int.__add__, '*': int.__mul__}

    def __init__(self, expr, prec=False):
        self.expr = expr
        self.prec = prec

    def eval_(self, start=0):
        res, op, pos = [], '+', start
        while pos < len(self.expr):
            ch = self.expr[pos]
            if ch.isdigit():
                res.append((op, int(ch)))
            elif ch in '+*':
                op = ch
            elif ch == '(':
                n, pos = self.eval_(pos + 1)
                res.append((op, n))
            elif ch == ')':
                return self.fold(res), pos
            pos += 1
        return self.fold(res)

    def fold(self, data):
        if self.prec:
            data = self.prepare(data)
        acc = 0
        for op, num in data:
            acc = self.ops[op](acc, num)
        return acc

    def prepare(self, data):
        res = data[:1]
        for item in data[1:]:
            if item[0] == '+':
                value = res[-1][1] + item[1]
                res[-1] = res[-1][0], value
            else:
                res.append(item)
        return res

input_ = list(open('day18.txt'))
sum_all = lambda prec: sum(Calc(line, prec).eval_() for line in input_)
print(sum_all(False))
print(sum_all(True))
