use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::from_fn;

fn calories(reader: impl BufRead) -> impl Iterator<Item = u32> {
    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    let calories = from_fn(move || {
        if lines.peek().is_none() {
            None
        } else {
            let calories = lines.by_ref().map_while(|line| line.trim().parse::<u32>().ok()).sum();
            Some(calories)
        }
    });
    calories
}

fn part1(reader: impl BufRead) -> u32 {
    calories(reader).max().unwrap_or(0)
}

fn part2(reader: impl BufRead) -> u32 {
    let mut calories = calories(reader).collect::<Vec<_>>();
    calories.sort();
    let total : u32 = calories.iter().rev().take(3).sum();
    total
}

fn main() {
    let infile = "input/day1.txt";
    let mode = std::env::args().nth(1);
    if mode.is_none() || mode == Some(String::from("--part1")) {
        let infile = File::open(infile).unwrap();
        let reader = BufReader::new(infile);
        println!("part1: {:?}", part1(reader));
    }
    if mode.is_none() || mode == Some(String::from("--part2")) {
        let infile = File::open(infile).unwrap();
        let reader = BufReader::new(infile);
        println!("part2: {:?}", part2(reader));
    }
}


#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn p1_zero_elf() {
        let infile = r#""#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 0u32);

        let infile =
r#"

"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 0u32);
    }

    #[test]
    fn p1_one_elf() {
        let infile = r#"10"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 10u32);

        let infile =
r#"
10
"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 10u32);

        let infile =
r#"10

"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 10u32);

        let infile =
r#"

10"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 10u32);
    }

    #[test]
    fn p1_many_elves() {
        let infile =
r#"
10
20


30

40
"#;
        assert_eq!(part1(BufReader::new(infile.as_bytes())), 40u32);
    }

    #[test]
    fn p2_insufficient_elf() {
        let infile = r#""#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 0u32);

        let infile =
r#"

"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 0u32);

        let infile =
r#"
10
"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 10u32);

        let infile =
r#"
10

20
"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 30u32);

        let infile =
r#"
10

20

30
"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 60u32);

        let infile =
r#"
10

20
30

40

"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 100u32);
    }

    #[test]
    fn p2_sufficient_elf() {
        let infile =
r#"
10

20

30

40

"#;
        assert_eq!(part2(BufReader::new(infile.as_bytes())), 90u32);
    }
}

