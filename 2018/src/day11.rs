use std::cmp;

type Point = (u32, u32);

struct Grid {
  cells: Vec<Vec<i32>>,
}

impl Grid {
  fn new(grid_id: u32) -> Grid {
    let size = 300;
    let cells = (0..size).map(|y| {
      (0..size).map(|x| power_level(grid_id, (x, y))).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    Grid { cells }
  }

  fn find_square(&self, size: u32) -> Point {
    let n = self.cells.len() as u32 - size;
    (0..=n).flat_map(|x| (0..=n).map(move |y| {
      let res = (0..size).flat_map(|dx| (0..size).map(move |dy| {
        self.cells[(y + dy) as usize][(x + dx) as usize]
      })).sum::<i32>();
      (res, (x, y))
    })).max().map(|t| t.1).unwrap()
  }

  fn find_largest(&self) -> (u32, u32, u32) {
    let n = self.cells.len() as u32;
    (0..n).flat_map(|x| (0..n).flat_map(move |y| {
      let mut acc = 0_i32;
      (1..=cmp::min(n - x, n - y)).map(move |d| {
        let (px, py) = ((x + d - 1) as usize, (y + d - 1) as usize);
        acc += self.cells[py][px] + (0..d).map(|i| {
          self.cells[py][(x + i) as usize] + self.cells[(y + i) as usize][px]
        }).sum::<i32>();
        (-acc, n - d, (x, y, d))
      })
    })).min().map(|t| t.2).unwrap()
  }
}

fn power_level(grid_id: u32, (x, y): Point) -> i32 {
  let rack_id = x + 10;
  let res = (y * rack_id + grid_id) * rack_id;
  (res as i32 / 100) % 10 - 5
}

pub fn run(content: &str) {
  let grid_id = content.trim().parse::<u32>().unwrap();
  let res1 = Grid::new(grid_id).find_square(3);
  let res2 = Grid::new(grid_id).find_largest();
  println!("{:?} {:?}", res1, res2);
}

#[cfg(test)]
mod tests {
  #[test]
  fn small() {
    assert_eq!(super::Grid::new(18).find_square(3), (33, 45));
    assert_eq!(super::Grid::new(42).find_square(3), (21, 61));
  }

  #[test]
  fn large() {
    //assert_eq!(super::Grid::new(18).find_largest(), (90, 269, 16));
    //assert_eq!(super::Grid::new(42).find_largest(), (232, 251, 12));
  }
}
