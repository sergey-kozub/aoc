use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

type Position = (u16, u16);
type Path = (usize, KeySet);

fn adjacent((x, y): Position) -> [Position; 4] {
    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct KeySet(u32);
type KeyIndex = u8;

impl KeySet {
    fn has(&self, key: KeyIndex) -> bool {
        self.0 & 1_u32 << key != 0
    }

    fn has_all(&self, other: &KeySet) -> bool {
        self.0 & other.0 == other.0
    }

    fn add(&mut self, key: KeyIndex) {
        self.0 |= 1_u32 << key;
    }
}

impl fmt::Debug for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result: String = (0..32).filter(|&n| self.has(n))
            .map(|n| (b'a' + n) as char).collect();
        write!(f, "{}", result)
    }
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PosPack(u64);

impl PosPack {
    fn new(data: &[u8]) -> PosPack {
        PosPack(data.iter().enumerate().map(
            |(k, v)| (*v as u64) << k * 8).sum::<u64>())
    }

    fn get(&self, index: u8) -> KeyIndex {
        ((self.0 >> index * 8) & u8::MAX as u64) as KeyIndex
    }

    fn set(&mut self, index: u8, value: KeyIndex) {
        self.0 &= !((u8::MAX as u64) << index * 8);
        self.0 |= (value as u64) << index * 8;
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Space,
    Wall,
    Key(u8),
    Door(u8),
    Start(u8),
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    keys: HashMap<KeyIndex, Position>,
    doors: HashMap<KeyIndex, Position>,
    start: Vec<Position>,
    paths: HashMap<(KeyIndex, KeyIndex), Vec<Path>>,
}

impl Maze {
    fn from(text: &str) -> Maze {
        let lines: Vec<&str> = text.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut data: Vec<Cell> = Vec::with_capacity(width * height);
        let mut keys: HashMap<KeyIndex, Position> = HashMap::new();
        let mut doors: HashMap<KeyIndex, Position> = HashMap::new();
        let mut start: Vec<Position> = Vec::new();

        for (y, line) in lines.into_iter().enumerate() {
            for (x, ch) in line.as_bytes().into_iter().enumerate() {
                let pos = (x as u16, y as u16);
                data.push(match ch {
                    b'.' => Cell::Space,
                    b'#' => Cell::Wall,
                    b'a'..=b'z' => {
                        let n = ch - b'a';
                        keys.insert(n, pos);
                        Cell::Key(n)
                    },
                    b'A'..=b'Z' => {
                        let n = ch - b'A';
                        doors.insert(n, pos);
                        Cell::Door(n)
                    },
                    b'@' => {
                        let n = start.len() as KeyIndex;
                        start.push(pos);
                        Cell::Start(n)
                    },
                    _ => panic!("unknown symbol"),
                });
            }
        }

        let paths = HashMap::new();
        Maze { width, height, data, keys, doors, start, paths }
    }

    fn cell(&self, (x, y): Position) -> &Cell {
        &self.data[y as usize * self.width + x as usize]
    }

    fn explore(&self, origin: Cell, target: KeyIndex) -> Vec<Path> {
        let mut result: Vec<Path> = Vec::new();
        let mut visited: HashMap<Position, Vec<KeySet>> = HashMap::new();
        let mut heap: BinaryHeap<(isize, Position, KeySet)> = BinaryHeap::new();

        let start = match origin {
            Cell::Key(n) => *self.keys.get(&n).unwrap(),
            Cell::Start(n) => self.start[n as usize],
            _ => panic!(),
        };
        heap.push((0, start, KeySet(0)));

        while let Some((rank, pos, mut doors)) = heap.pop() {
            match *self.cell(pos) {
                Cell::Key(n) if n == target => {
                    result.push((-rank as usize, doors));
                    continue;
                },
                Cell::Door(n) => doors.add(n),
                _ => (),
            }

            let dset = visited.entry(pos).or_default();
            if !dset.iter().any(|d| doors.has_all(d)) {
                dset.push(doors);
                for next in &adjacent(pos) {
                    if !matches!(self.cell(*next), Cell::Wall) {
                        heap.push((rank - 1, *next, doors));
                    }
                }
            }
        }
        result
    }

    fn explore_all(&mut self) {
        for &target in self.keys.keys() {
            for &source in self.keys.keys() {
                self.paths.insert((source, target),
                    self.explore(Cell::Key(source), target));
            }
            for i in 0..self.start.len() as KeyIndex {
                self.paths.insert((u8::MAX - i, target),
                    self.explore(Cell::Start(i), target));
            }
        }
    }

    fn search(&self) -> Option<usize> {
        let num_bots = self.start.len() as KeyIndex;
        let num_keys = self.keys.len() as KeyIndex;
        let all_keys = KeySet((1_u32 << num_keys) - 1);

        let mut visited: HashSet<(PosPack, KeySet)> = HashSet::new();
        let mut heap: BinaryHeap<(isize, PosPack, KeySet)> = BinaryHeap::new();
        let init: Vec<u8> = (0..num_bots).map(|n| u8::MAX - n).collect();
        heap.push((0, PosPack::new(&init), KeySet(0)));

        while let Some((rank, packed, keys)) = heap.pop() {
            if visited.contains(&(packed, keys)) { continue; }
            visited.insert((packed, keys));
            if keys == all_keys {
                return Some(-rank as usize);
            }

            for bot in 0..num_bots {
                let source = packed.get(bot);
                for target in 0..num_keys {
                    if !keys.has(target) {
                        let paths = self.paths.get(&(source, target)).unwrap();
                        let first = paths.iter().find(|(_, doors)| keys.has_all(&doors));
                        if let Some((dist, _)) = first {
                            let mut new_packed = packed.clone();
                            new_packed.set(bot, target);
                            let mut new_keys = keys.clone();
                            new_keys.add(target);
                            heap.push((rank - *dist as isize, new_packed, new_keys));
                        }
                    }
                }
            }
        }
        None
    }

    fn explore_search(&mut self) -> usize {
        self.explore_all();
        self.search().unwrap()
    }

    fn cell_mut(&mut self, (x, y): Position) -> &mut Cell {
        &mut self.data[y as usize * self.width + x as usize]
    }

    fn patch(&mut self) {
        assert_eq!(self.start.len(), 1);
        let (x, y) = self.start.pop().unwrap();
        for dy in -1..=1 { for dx in -1..=1 {
            let pos = (x + dx as u16, y + dy as u16);
            if dy != 0 && dx != 0 {
                let n = self.start.len() as KeyIndex;
                self.start.push(pos);
                *self.cell_mut(pos) = Cell::Start(n);
            } else {
                *self.cell_mut(pos) = Cell::Wall;
            }
        }}
    }
}

pub fn run(content: &str) {
    let mut maze = Maze::from(content);
    let res_1 = maze.explore_search();
    maze.patch();
    maze.paths.clear();
    let res_2 = maze.explore_search();
    println!("{} {}", res_1, res_2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let mut maze_1 = super::Maze::from("\
            ########################\n\
            #f.D.E.e.C.b.A.@.a.B.c.#\n\
            ######################.#\n\
            #d.....................#\n\
            ########################");
        assert_eq!(86, maze_1.explore_search());

        let mut maze_2 = super::Maze::from("\
            ########################\n\
            #...............b.C.D.f#\n\
            #.######################\n\
            #.....@.a.B.c.d.A.e.F.g#\n\
            ########################");
        assert_eq!(132, maze_2.explore_search());

        let mut maze_3 = super::Maze::from("\
            #################\n\
            #i.G..c...e..H.p#\n\
            ########.########\n\
            #j.A..b...f..D.o#\n\
            ########@########\n\
            #k.E..a...g..B.n#\n\
            ########.########\n\
            #l.F..d...h..C.m#\n\
            #################");
        assert_eq!(136, maze_3.explore_search());

        let mut maze_4 = super::Maze::from("\
            ########################\n\
            #@..............ac.GI.b#\n\
            ###d#e#f################\n\
            ###A#B#C################\n\
            ###g#h#i################\n\
            ########################");
        assert_eq!(81, maze_4.explore_search());
    }

    #[test]
    fn part2() {
        let mut maze_1 = super::Maze::from("\
            ###############\n\
            #d.ABC.#.....a#\n\
            ######@#@######\n\
            ###############\n\
            ######@#@######\n\
            #b.....#.....c#\n\
            ###############");
        assert_eq!(24, maze_1.explore_search());

        let mut maze_2 = super::Maze::from("\
            #############\n\
            #DcBa.#.GhKl#\n\
            #.###@#@#I###\n\
            #e#d#####j#k#\n\
            ###C#@#@###J#\n\
            #fEbA.#.FgHi#\n\
            #############");
        assert_eq!(32, maze_2.explore_search());

        let mut maze_3 = super::Maze::from("\
            #############\n\
            #g#f.D#..h#l#\n\
            #F###e#E###.#\n\
            #dCba@#@BcIJ#\n\
            #############\n\
            #nK.L@#@G...#\n\
            #M###N#H###.#\n\
            #o#m..#i#jk.#\n\
            #############");
        assert_eq!(72, maze_3.explore_search());
    }
}
