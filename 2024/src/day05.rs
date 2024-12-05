use std::collections::HashSet;

fn parse_rules(text: &str) -> Vec<(u32, u32)> {
    text.lines().map(|line| {
        let (l, r) = line.split_once('|').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
    }).collect()
}

fn parse_pages(text: &str) -> Vec<Vec<u32>> {
    text.lines().map(|line| {
        line.split(',').map(|s| s.parse().unwrap()).collect()
    }).collect()
}

fn is_ordered(rules: &[(u32, u32)], items: &[u32]) -> bool {
    let pos = |v| items.iter().position(|&x| x == v);
    !rules.iter().any(|(k, v)| {
        match pos(*k) {
            Some(p) => pos(*v).unwrap_or(items.len()) < p,
            None => false,
        }
    })
}

fn order(rules: &[(u32, u32)], items: &[u32]) -> Vec<u32> {
    let mut pool = HashSet::<u32>::from_iter(items.iter().copied());
    let mut result = vec![];
    while !pool.is_empty() {
        let select = *pool.iter().filter(|&&n| {
            !rules.iter().any(|(k, v)| n == *v && pool.contains(k))
        }).next().unwrap();
        result.push(select);
        pool.remove(&select);
    }
    result
}

pub fn run(content: &str) {
    let (s1, s2) = content.split_once("\n\n").unwrap();
    let (rules, pages) = (parse_rules(s1), parse_pages(s2));
    let (ordered, unordered): (Vec<_>, Vec<_>) = pages.into_iter()
        .partition(|a| is_ordered(&rules, &a));
    let sum = |x: &Vec<Vec<_>>| x.iter().map(|a| a[a.len() / 2]).sum::<u32>();
    let fixed = unordered.into_iter()
        .map(|a| order(&rules, &a)).collect::<Vec<_>>();
    println!("{} {}", sum(&ordered), sum(&fixed));
}

#[cfg(test)]
mod tests {
    const TEST_RULES: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13";

    const TEST_PAGES: &str = "\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47";

    #[test]
    fn small() {
        let rules = super::parse_rules(TEST_RULES);
        let pages = super::parse_pages(TEST_PAGES);
        let ordered = pages.iter().map(|a| super::is_ordered(&rules, a));
        assert_eq!(ordered.collect::<Vec<_>>(),
                   vec![true, true, true, false, false, false]);
    }

    #[test]
    fn large() {
        let rules = super::parse_rules(TEST_RULES);
        let pages = super::parse_pages(TEST_PAGES);
        assert_eq!(super::order(&rules, &pages[3]), vec![97, 75, 47, 61, 53]);
        assert_eq!(super::order(&rules, &pages[4]), vec![61, 29, 13]);
        assert_eq!(super::order(&rules, &pages[5]), vec![97, 75, 47, 29, 13]);
    }
}
