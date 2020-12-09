from collections import Counter

def find_weak(A, N):
    sums = Counter(A[i] + A[j] for i in range(N) for j in range(i + 1, N))
    for i in range(N, len(A)):
        if not sums[A[i]]: return A[i]
        sums.subtract(A[i - N] + A[j] for j in range(i - N + 1, i))
        sums.update(A[i] + A[j] for j in range(i - N + 1, i))

def find_range(A, W):
    psum, N = A[:1], len(A)
    for v in A[1:]: psum.append(psum[-1] + v)
    return [(i, j) for i in range(N) for j in range(i + 1, N) if psum[j] - psum[i] == W]

data = list(map(int, open("day9.txt")))
W = find_weak(data, 25)
i, j = find_range(data, W)[0]
part = data[i:j + 1]
print(W, min(part) + max(part))
