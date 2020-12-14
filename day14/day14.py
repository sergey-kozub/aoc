import re

program = []
for line in open("day14.txt"):
    m = re.match(r'mask = ([X01]+)|mem\[(\d+)\] = (\d+)', line)
    program.append(m.groups())

memory, or_mask, and_mask = {}, None, None
for mask, addr, value in program:
    if mask:
        and_mask = int(''.join('1' if c == 'X' else '0' for c in mask), 2)
        or_mask = int(mask.replace('X', '0'), 2)
    elif addr:
        memory[int(addr)] = (int(value) & and_mask) | or_mask
print(sum(memory.values()))

memory, addr_mask = {}, None
def write(addr, value):
    if 'X' in addr:
        write(addr.replace('X', '0', 1), value)
        write(addr.replace('X', '1', 1), value)
    else:
        memory[int(addr, 2)] = value
for mask, addr, value in program:
    if mask:
        addr_mask = mask
    elif addr:
        addr_1 = bin(int(addr))[2:].rjust(len(addr_mask), '0')
        addr_2 = ''.join(addr_1[k] if v == '0' else v for k, v in enumerate(addr_mask))
        write(addr_2, int(value))
print(sum(memory.values()))
