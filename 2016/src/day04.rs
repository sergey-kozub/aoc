use std::collections::HashMap;

fn parse(name: &str) -> (&str, u32, &str) {
    let (name, tail) = name.rsplit_once('-').unwrap();
    let (l, r) = tail.split_once('[').unwrap();
    let sector = l.parse::<u32>().unwrap();
    (name, sector, &r[..r.len() - 1])
}

fn real_room_id(text: &str) -> Option<u32> {
    let (name, sector, result) = parse(text);
    let mut count = HashMap::<char, i32>::new();
    for ch in name.chars() {
        if ch == '-' { continue; }
        count.entry(ch).and_modify(|n| *n += 1).or_insert(0);
    }
    let mut items = count.into_iter().collect::<Vec<_>>();
    items.sort_by_key(|x| (-x.1, x.0));
    let top5 = items.into_iter().take(5).map(|x| x.0).collect::<String>();
    if top5 == result {Some(sector)} else {None}
}

fn decrypt(text: &str) -> String {
    let (name, sector, _) = parse(text);
    let a_ = 'a' as u32;
    name.chars().map(|ch| {
        if ch == '-' { return ' '; }
        char::from_u32((ch as u32 - a_ + sector) % 26 + a_).unwrap()
    }).collect::<String>()
}

pub fn run(content: &str) {
    let rooms = content.lines().collect::<Vec<_>>();
    let search = "northpole object storage";
    let v1 = rooms.iter().filter_map(|&s| real_room_id(s)).sum::<u32>();
    let v2 = rooms.iter().filter_map(|&s| {
        let sector = real_room_id(s)?;
        if decrypt(s) == search {Some(sector)} else {None}
    }).next().unwrap();
    println!("{} {}", v1, v2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert!(super::real_room_id("aaaaa-bbb-z-y-x-123[abxyz]").is_some());
        assert!(super::real_room_id("a-b-c-d-e-f-g-h-987[abcde]").is_some());
        assert!(super::real_room_id("not-a-real-room-404[oarel]").is_some());
        assert!(super::real_room_id("totally-real-room-200[decoy]").is_none());
    }

    #[test]
    fn large() {
        assert_eq!(super::decrypt("qzmt-zixmtkozy-ivhz-343[]"),
                   "very encrypted name");
    }
}
