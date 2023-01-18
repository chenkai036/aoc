use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

struct Offset {
    row: isize,
    col: isize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn array() -> &'static [Direction; 4] {
        &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    const fn size() -> usize {
        Direction::array().len()
    }

    const fn offset(&self) -> Offset {
        match self {
            Direction::North => Offset { row: -1, col: 0},
            Direction::South => Offset { row: 1, col: 0},
            Direction::East => Offset { row: 0, col: 1},
            Direction::West => Offset { row: 0, col: -1},
        }
    }

    const fn value(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }
}

type Height = u8;
type Vec3D<T> = Vec<Vec<Vec<T>>>;
type State = (Height, bool); // (the height observed at current coordinate and direction so far,
                             // whether the tree is visible at the current row, col and direction)

struct Grid {
    value: Vec<Vec<Height>>,
    shape: (usize, usize),
}

impl<T: BufRead> From<T> for Grid {
    fn from(reader: T) -> Self {
        let value : Vec<Vec<Height>> = reader.lines().map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|height| height.to_digit(10).unwrap() as Height)
                .collect()
        }).collect();
        let shape = (value.len(), value.get(0).map_or(0, |row| row.len()));
        Grid { value, shape }
    }
}

impl Grid {
    fn tree_on_edge(&self, row: usize, col: usize, direction: Direction) -> bool {
        let (rows, cols) = self.shape;
        match direction {
            Direction::North => row == 0,
            Direction::South => row == rows - 1,
            Direction::West => col == 0,
            Direction::East => col == cols - 1,
        }
    }

    fn visible_towards(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
        states: &mut Vec3D<Option<State>>)
    -> State {
        if states[row][col][direction.value()].is_none() {
            if self.tree_on_edge(row, col, direction) {
                return (self.value[row][col], true);
            }
            let offset = direction.offset();
            let prev_row = (row as isize + offset.row) as usize;
            let prev_col = (col as isize + offset.col) as usize;
            let (prev_height, _) = self.visible_towards(prev_row, prev_col, direction, states);
            let this_height = max(prev_height, self.value[row][col]);
            states[row][col][direction.value()] = Some((this_height, prev_height < self.value[row][col]));
        }
        states[row][col][direction.value()].unwrap()
    }
}

// dynamic programming:
//   states[row][col][North] = max(states[row - 1][col][North], grid[row][col])
fn part1_v2(reader: impl BufRead) -> u32 {
    let grid = Grid::from(reader);
    let (rows, cols) = grid.shape;
    let mut result = 0;
    let mut states = vec![vec![vec![None; Direction::size()]; cols]; rows];

    // initial state
    for row in 0..rows {
        states[row][0][Direction::West.value()] = Some((grid.value[row][0], true));
        states[row][cols - 1][Direction::East.value()] = Some((grid.value[row][cols - 1], true));
    }
    for col in 0..cols {
        states[0][col][Direction::North.value()] = Some((grid.value[0][col], true));
        states[rows - 1][col][Direction::South.value()] = Some((grid.value[rows - 1][col], true));
    }

    for row in 0..rows {
        for col in 0..cols {
            let is_visible = Direction::array().iter().any(|direction| {
                let (_, visible) = grid.visible_towards(row, col, *direction, &mut states);
                visible
            });
            if is_visible {
                result += 1;
            }
        }
    }

    result
}

fn part1(reader: impl BufRead) -> u32 {
    let grid = Grid::from(reader);
    let mut result = 0;
    let (rows, cols) = grid.shape;

    for row in 0..rows {
        for col in 0..cols {
            let mut is_visible = false;
            if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
                is_visible = true;
            } else {
                let height = grid.value[row][col];
                is_visible |= (0..row).all(|r| grid.value[r][col] < height)
                           || (row+1..rows).all(|r| grid.value[r][col] < height)
                           || (0..col).all(|c| grid.value[row][c] < height)
                           || (col+1..cols).all(|c| grid.value[row][c] < height);
            }
            if is_visible {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let reader = BufReader::new(File::open("input/day8.txt").unwrap());
    println!("part1 = {}", part1(reader));
    let reader = BufReader::new(File::open("input/day8.txt").unwrap());
    println!("part1 using lookup = {}", part1_v2(reader));
}

#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn example() {
        let input =
r#"
30373
25512
65332
33549
35390
"#.trim();
        let reader = BufReader::new(input.as_bytes());
        assert_eq!(part1_v2(reader), 21);
    }

//  #[bench(loop)]
//  fn use_loop(&mut Bencher) {
//      let reader = BufReader::new(File::open("input/day8.txt").unwrap());
//      println!("part1 = {}", part1(reader));
//  }

}

