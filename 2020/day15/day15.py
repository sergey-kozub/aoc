
def game(A, N):
  spoken = {v: k + 1 for k, v in enumerate(A)}
  for i in range(len(A), N):
    prev = spoken.get(A[-1])
    A.append(i - prev if prev is not None else 0)
    spoken[A[-2]] = i
  return A[-1]

S = open('day15.txt').read()
A = list(map(int, S.split(',')))
print(game(A, 2020))
print(game(A, 30000000))
