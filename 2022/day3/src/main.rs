use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::from_fn;

fn priority(badge: char) -> u32 {
    if badge.is_lowercase() {
        badge as u32 - 'a' as u32 + 1
    } else if badge.is_uppercase() {
        badge as u32 - 'A' as u32 + 27
    } else {
        0u32
    }
}

fn part1(reader: impl BufRead) -> u32 {
    let priorities = reader.lines().map(|line| {
        let line = line.unwrap();
        let room_size = line.len() / 2;
        let left_room : HashSet<char> = HashSet::from_iter(line.chars().take(room_size));
        let right_room : HashSet<char> = HashSet::from_iter(line.chars().skip(room_size));
        left_room.intersection(&right_room).fold(0u32, |accum, value| {
            accum + priority(*value)
        })
    });
    priorities.sum()
}

fn item_by_hashset(lines: impl Iterator<Item = String>) -> char {
    let initial : HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    let items = lines.map(|line| {
        HashSet::from_iter(line.chars()) as HashSet<char>
    });
    let common = items.fold(initial, |mut accum, value| {
        accum.retain(|item| value.contains(item));
        accum
    });
    *common.iter().next().unwrap()
}

fn item_to_index(item: char) -> usize {
    if item.is_lowercase() {
        item as usize - 'a' as usize
    } else {
        item as usize - 'A' as usize + 26
    }
}

fn index_to_item(index: usize) -> char {
    if index < 26 {
        ('a' as u8 + index as u8) as char
    } else {
        ('A' as u8 + index as u8 - 26) as char
    }
}

fn item_by_array(lines: impl Iterator<Item = String>) -> char {
    let mut alphabet = [0u32; 26 * 2];
    lines.for_each(|line| {
        let items = line.chars().fold([false; 26 * 2], |mut accum, item| {
            let index = item_to_index(item);
            accum[index] |= true;
            accum
        });
        alphabet.iter_mut().zip(items).for_each(|(item, flag)| {
            if flag {
                *item += 1;
            }
        });
    });
    let index = alphabet.iter().position(|v| *v == 3).unwrap();
    let item = index_to_item(index);
    item
}

fn foreach_elf_group<GroupFn>(reader: impl BufRead, apply: GroupFn) -> u32
where GroupFn: Fn(Box<dyn Iterator<Item = String> + '_>) -> char {
    let mut lines = reader.lines().map(|line| line.unwrap()).peekable();
    let groups = from_fn(move || {
        if lines.peek().is_none() {
            None
        } else {
            Some(apply(Box::new(lines.by_ref().filter(|line| line.is_empty() == false).take(3))))
        }
    });
    groups.map(|item| priority(item)).sum()
}

fn part2_v1(reader: impl BufRead) -> u32 {
    foreach_elf_group(reader, |egrp| item_by_hashset(egrp))
}

fn part2_v2(reader: impl BufRead) -> u32 {
    foreach_elf_group(reader, |egrp| item_by_array(egrp))
}

fn main() {
    let result = part1(BufReader::new(File::open("input/day3.txt").unwrap()));
    println!("part1: {result}");
    let result = part2_v1(BufReader::new(File::open("input/day3.txt").unwrap()));
    println!("part2 v1: {result}");
    let result = part2_v2(BufReader::new(File::open("input/day3.txt").unwrap()));
    println!("part2 v2: {result}");
}

#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn example() {
        let input =
r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
        let result = part1(BufReader::new(input.as_bytes()));
        assert_eq!(157, result);
        let result = part2_v1(BufReader::new(input.as_bytes()));
        assert_eq!(70, result);
        let result = part2_v2(BufReader::new(input.as_bytes()));
        assert_eq!(70, result);
    }
}

