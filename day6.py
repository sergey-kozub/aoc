data = open('day6.txt').read().rstrip().split('\n\n')
groups = [s.split('\n') for s in data]
count_any = lambda g: len(set(''.join(g)))
count_all = lambda g: len(set.intersection(*map(set, g)))
print(sum(map(count_any, groups)))
print(sum(map(count_all, groups)))
