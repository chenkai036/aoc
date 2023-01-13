use std::io::{BufRead, BufReader};
mod fs;
mod parse;
use crate::fs::Tree;

fn part1(mut reader: impl BufRead) -> usize {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let state = parse::parse(input).unwrap();
    let root = state.fs.root();
    root.fold(&state.fs, 0, &|accum, entry| {
        match entry {
            fs::Entry::File(_) => accum,
            fs::Entry::Directory(directory) => {
                let dsize = directory.handle.size(&state.fs); // FIXME inefficient
                if dsize <= 100_000 {
                    accum + dsize
                } else {
                    accum
                }
            }
        }
    })
}

fn part2(mut reader: impl BufRead) -> usize {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let state = parse::parse(input).unwrap();
    let root = state.fs.root();
    let mut dir_sizes = root.fold(&state.fs, vec![], &|mut accum, entry| {
        match entry {
            fs::Entry::File(_) => (),
            fs::Entry::Directory(directory) => {
                accum.push(directory.handle.size(&state.fs)); // FIXME inefficient
            }
        }
        accum
    });
    let used_size = dir_sizes.iter().max().unwrap();
    let (total_size, required_size) = (70_000_000usize, 30_000_000usize);
    let unused_size = total_size - used_size;
    if required_size <= unused_size {
        return 0; // no need to delete any directory
    }
    let freeup_size = required_size - unused_size;
    dir_sizes.sort();
    *dir_sizes.iter().skip_while(|dsize| **dsize < freeup_size).nth(0).unwrap_or(&0)
}

fn main() {
    let reader = BufReader::new(std::fs::File::open("input/day7.txt").unwrap());
    println!("part1 = {}", part1(reader));
    let reader = BufReader::new(std::fs::File::open("input/day7.txt").unwrap());
    println!("part2 = {}", part2(reader));
}

