use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
struct Section {
    lo: u32,
    hi: u32,
}

#[derive(Debug)]
struct ParseSectionError;

impl FromStr for Section {
    type Err = ParseSectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lo, hi) = s.split_once('-').ok_or(ParseSectionError)?;
        let lo = lo.parse().map_err(|_| ParseSectionError)?;
        let hi = hi.parse().map_err(|_| ParseSectionError)?;
        if hi < lo {
            Err(ParseSectionError)
        } else {
            Ok(Section {lo, hi})
        }
    }
}

impl Section {
    fn overlap(&self, other: &Section) -> bool {
        assert!(self.lo <= self.hi && other.lo <= other.hi);
        !(self.hi < other.lo || other.hi < self.lo)
    }

    fn cover(&self, other: &Section) -> bool {
        assert!(self.lo <= self.hi && other.lo <= other.hi);
        self.lo <= other.lo && other.hi <= self.hi
    }
}

fn foreach_elf_pair<PairFn>(reader: impl BufRead, apply: PairFn) -> u32
where PairFn: Fn(&Section, &Section) -> bool {
    let lines = reader.lines().filter_map(|line| {
        let line = line.unwrap();
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    });
    let assignments = lines.filter(|line| {
        let (left, right) = line.split_once(',').map(|(left, right)| {
            let left = Section::from_str(&left).unwrap();
            let right = Section::from_str(&right).unwrap();
            (left, right)
        }).unwrap();
        apply(&left, &right)
    });
    assignments.count() as u32
}

fn part1(reader: impl BufRead) -> u32 {
    foreach_elf_pair(reader, |left, right| left.cover(&right) || right.cover(&left))
}

fn part2(reader: impl BufRead) -> u32 {
    foreach_elf_pair(reader, |left, right| left.overlap(&right) || right.overlap(&left))
}

fn main() {
    let result = part1(BufReader::new(File::open("input/day4.txt").unwrap()));
    println!("part1: {result}");
    let result = part2(BufReader::new(File::open("input/day4.txt").unwrap()));
    println!("part2: {result}");
}

#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn test_example() {
        let input =
r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
        let reader = BufReader::new(input.as_bytes());
        assert_eq!(part1(reader), 2);
        let reader = BufReader::new(input.as_bytes());
        assert_eq!(part2(reader), 4);
    }

}

