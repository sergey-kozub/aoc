N, M = 7, 20201227

def transform(subj, loop):
    n = 1
    for _ in range(loop):
        n = n * subj % M
    return n

def reverse(res):
    n, s = 1, 0
    while n != res:
        n, s = n * N % M, s + 1
    return s

A, B = 8987316, 14681524
print(transform(B, reverse(A)))
print(transform(A, reverse(B)))
