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
                let dsize = directory.handle.size(&state.fs);
                if dsize <= 100_000 {
                    accum + dsize
                } else {
                    accum
                }
            }
        }
    })
}

fn main() {
    let reader = BufReader::new(std::fs::File::open("input/day7.txt").unwrap());
    println!("part1 = {}", part1(reader));
}

