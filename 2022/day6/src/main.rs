use std::fs::File;
use std::cmp::min;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn find_marker(mut reader: impl BufRead, nchar: usize) -> u32 {
    let mut signal = String::new();
    reader.read_line(&mut signal).expect("invalid signal input");

    let mut window = HashMap::<char, u32>::new();
    signal.chars().take(nchar).for_each(|c| {
        if let Some(count) = window.insert(c, 1) {
            window.insert(c, count + 1);
        }
    });

    (signal.chars()
          .zip(signal.chars().skip(nchar))
          .take_while(|(slow, fast)| {
              if window.len() == nchar {
                  return false;
              }
              if let Some(count) = window.remove(slow) {
                  if count != 1 {
                      window.insert(*slow, count - 1);
                  }
              }
              if let Some(count) = window.insert(*fast, 1) {
                  window.insert(*fast, count + 1);
              }
              true
          })
          .count() + min(nchar, signal.len())) as u32
}

fn part1(reader: impl BufRead) -> u32 {
    find_marker(reader, 4)
}

fn part2(reader: impl BufRead) -> u32 {
    find_marker(reader, 14)
}

fn main() {
    let result = part1(BufReader::new(File::open("input/day6.txt").unwrap()));
    println!("part1 = {result}");
    let result = part2(BufReader::new(File::open("input/day6.txt").unwrap()));
    println!("part2 = {result}");
}

#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn test_example() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 7);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 5);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 6);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 10);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 11);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 26);
    }

    #[test]
    fn test_short_signal() {
        let input = "xyz";
        assert_eq!(part1(BufReader::new(input.as_bytes())), 3);
        assert_eq!(part2(BufReader::new(input.as_bytes())), 3);
    }

}

