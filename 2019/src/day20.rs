use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

type Position = (i32, i32);

fn adjacent((x, y): Position) -> [Position; 4] {
    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct PortalKey([u8; 2]);

impl fmt::Debug for PortalKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Unknown,
    Space,
    Wall,
    Portal(PortalKey, bool),
}

struct Grid {
    width: usize,
    data: Vec<Cell>,
    portals: HashMap<PortalKey, Vec<Position>>,
}

impl Grid {
    fn from(text: &str) -> Grid {
        let lines: Vec<&[u8]> = text.lines().map(|s| s.as_bytes()).collect();
        let width = lines.iter().map(|a| a.len()).max().unwrap();
        let height = lines.len();
        let mut data = vec![Cell::Unknown; width * height];
        let mut letters: HashMap<usize, u8> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                data[y * width + x] = match ch {
                    b'.' => Cell::Space,
                    b'#' => Cell::Wall,
                    b'A'..=b'Z' => {
                        letters.insert(y * width + x, *ch);
                        Cell::Unknown
                    },
                    _ => Cell::Unknown,
                };
            }
        }

        let mut portals: HashMap<PortalKey, Vec<Position>> = HashMap::new();
        for (&idx, &letter) in letters.iter() {
            let mut portal: Option<(PortalKey, usize)> = None;
            for &offset in [1, width].iter() {
                if idx >= offset && idx < data.len() - offset {
                    let (prev, next) = (idx - offset, idx + offset);
                    if letters.contains_key(&prev) && matches!(data[next], Cell::Space) {
                        let key = PortalKey([*letters.get(&prev).unwrap(), letter]);
                        portal = Some((key, next));
                    } else if letters.contains_key(&next) && matches!(data[prev], Cell::Space) {
                        let key = PortalKey([letter, *letters.get(&next).unwrap()]);
                        portal = Some((key, prev));
                    }
                }
            }
            if let Some((key, idx)) = portal {
                let pos = ((idx % width) as i32, (idx / width) as i32);
                let outer = pos.0 == 2 || pos.0 == width as i32 - 3 ||
                            pos.1 == 2 || pos.1 == height as i32 - 3;
                data[idx] = Cell::Portal(key, outer);
                portals.entry(key).or_default().push(pos);
            }
        }
        Grid { width, data, portals }
    }

    fn cell(&self, (x, y): Position) -> &Cell {
        &self.data[y as usize * self.width + x as usize]
    }

    fn search(&self, levels: Option<usize>) -> Option<usize> {
        let start = self.portals.get(&PortalKey(*b"AA")).unwrap()[0];
        let end = self.portals.get(&PortalKey(*b"ZZ")).unwrap()[0];

        let mut visited: HashSet<(isize, Position)> = HashSet::new();
        let mut heap: BinaryHeap<(isize, isize, Position)> = BinaryHeap::new();
        heap.push((0, 0, start));

        while let Some((level, rank, pos)) = heap.pop() {
            if visited.contains(&(level, pos)) { continue; }
            visited.insert((level, pos));

            for &next in adjacent(pos).iter() {
                match *self.cell(next) {
                    Cell::Space => heap.push((level, rank - 1, next)),
                    Cell::Portal(key, outer) => {
                        let pair = self.portals.get(&key).unwrap();
                        if pair.len() == 1 {
                            if level == 0 && next == end {
                                return Some(-rank as usize + 1);
                            } else {
                                continue;
                            }
                        }
                        let next = pair[if pair[0] == next {1} else {0}];
                        match levels {
                            Some(limit) if (-level as usize) < limit => {
                                if level != 0 || !outer {
                                    let delta = if outer {1} else {-1};
                                    heap.push((level + delta, rank - 2, next));
                                }
                            },
                            None => heap.push((level, rank - 2, next)),
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        }
        None
    }

    fn search_deep(&self) -> Option<usize> {
        self.search(Some(self.portals.len()))
    }
}

pub fn run(content: &str) {
    let grid = Grid::from(content);
    let score_1 = grid.search(None).unwrap();
    let score_2 = grid.search_deep().unwrap();
    println!("{} {}", score_1, score_2);
}

#[cfg(test)]
mod tests {
    fn input_1() -> &'static str { "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z".trim_matches('\n') }

    fn input_2() -> &'static str { "
                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P".trim_matches('\n') }

    fn input_3() -> &'static str { "
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M".trim_matches('\n') }

    #[test]
    fn part1() {
        let grid_1 = super::Grid::from(input_1());
        assert_eq!(Some(23), grid_1.search(None));

        let grid_2 = super::Grid::from(input_2());
        assert_eq!(Some(58), grid_2.search(None));
    }

    #[test]
    fn part2() {
        let grid_1 = super::Grid::from(input_1());
        assert_eq!(Some(26), grid_1.search_deep());

        let grid_2 = super::Grid::from(input_2());
        assert_eq!(None, grid_2.search_deep());

        let grid_3 = super::Grid::from(input_3());
        assert_eq!(Some(396), grid_3.search_deep());
    }
}
