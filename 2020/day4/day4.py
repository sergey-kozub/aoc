import re

rules = {
    'byr': lambda s: s.isdigit() and 1920 <= int(s) <= 2002,
    'iyr': lambda s: s.isdigit() and 2010 <= int(s) <= 2020,
    'eyr': lambda s: s.isdigit() and 2020 <= int(s) <= 2030,
    'hgt': lambda s: s[:-2].isdigit() and (
        (s.endswith('cm') and 150 <= int(s[:-2]) <= 193) or
        (s.endswith('in') and 59 <= int(s[:-2]) <= 76)),
    'hcl': lambda s: re.match(r'#[0-9a-f]{6}', s),
    'ecl': lambda s: re.match(r'amb|blu|brn|gry|grn|hzl|oth', s),
    'pid': lambda s: s.isdigit() and len(s) == 9,
}

data = [dict(re.findall(r'([a-z]{3}):(\S+)', block))
        for block in open('day4.txt').read().split('\n\n')]
print(sum(all(k in item for k in rules) for item in data))
print(sum(all(v(item.get(k, '')) for k, v in rules.items()) for item in data))
