from collections import deque

input_ = open("day22.txt").read().rstrip()
player1, player2 = [list(map(int, x.split('\n')[1:]))
                    for x in input_.split('\n\n')]

def play(player1, player2, recursive):
    played = set()
    p1, p2 = deque(player1), deque(player2)
    while p1 and p2:
        state = (len(p1),) + tuple(p1 + p2)
        if state in played: break
        c1, c2 = p1.popleft(), p2.popleft()
        if recursive and len(p1) >= c1 and len(p2) >= c2:
            s1, s2 = list(p1)[:c1], list(p2)[:c2]
            t1, t2 = play(s1, s2, True)
            win = len(t1) > 0
        else:
            win = c1 > c2
        if win:
            p1.extend((c1, c2))
        else:
            p2.extend((c2, c1))
        played.add(state)
    return p1, p2

score = lambda r: sum(v * (k + 1) for k, v in enumerate(reversed(r[0] or r[1])))
print(score(play(player1, player2, False)))
print(score(play(player1, player2, True)))
