program = []
for line in open('day8.txt'):
    m = line.split(' ')
    program.append((m[0], int(m[1])))

def emulate(program):
    acc, ptr, visited = 0, 0, set()
    while ptr < len(program) and ptr not in visited:
        visited.add(ptr)
        instr, value = program[ptr]
        if instr == 'jmp':
            ptr += value
            continue
        if instr == 'acc':
            acc += value
        ptr += 1
    return acc, ptr in visited
print(emulate(program)[0])

for idx, (instr, value) in enumerate(program):
    if instr in ('jmp', 'nop'):
        line = ('jmp' if instr != 'jmp' else 'nop', value)
        test = program[:idx] + [line] + program[idx + 1:]
        acc, loop = emulate(test)
        if not loop: break
print(acc)
