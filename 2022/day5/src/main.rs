use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

#[derive(Debug, Default)]
struct CrateStack(Vec<char>);

#[derive(Debug)]
struct CrateStackVec {
    array: Vec<CrateStack>
}

impl<T: BufRead> From<&mut Lines<T>> for CrateStackVec {

    fn from(lines: &mut Lines<T>) -> Self {
        let mut result = CrateStackVec { array: vec![] };
        let mut nvec = 0usize;
        let lines = lines.map(|line| line.unwrap());
        let layers = lines.take_while(|line| !line.is_empty()).map(|line| {
            if nvec == 0 {
                nvec = (line.len() + 1) / 4;
            }

            let chars = line.chars().collect::<Vec<char>>();
            let mut layer = vec![];
            for i in 0..nvec {
                let value = chars[i * 4 + 1];
                if value.is_whitespace() || value.is_digit(10) {
                    layer.push(None);
                } else if value.is_alphabetic() {
                    layer.push(Some(value))
                }
            }

            layer
        }).collect::<Vec<_>>();

        result.array.resize_with(nvec, || CrateStack(vec![]));
        for layer in layers {
            for i in 0..nvec {
                if let Some(val) = layer[i] {
                    result.array[i].0.insert(0, val);
                }
            }
        }
        result
    }

}

#[derive(Debug)]
struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}

#[derive(Debug)]
struct InvalidInstructionErr;

type Instructions = Vec<Instruction>;

impl FromStr for Instruction {

    type Err = InvalidInstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.split_whitespace().collect::<Vec<_>>();
        let count = cmd[1].parse::<u8>().map_err(|_| InvalidInstructionErr)?;
        let from = cmd[3].parse::<u8>().map_err(|_| InvalidInstructionErr)?;
        let to = cmd[5].parse::<u8>().map_err(|_| InvalidInstructionErr)?;
        Ok(Instruction {count, from, to})
    }
}

fn run<MoveCrateFn>(reader: impl BufRead, apply: MoveCrateFn) -> String
where MoveCrateFn: Fn(&mut CrateStackVec, &Instruction) {
    let mut lines = reader.lines();
    let mut stack_vec = CrateStackVec::from(&mut lines);
    let instructions = lines.map(|line| {
        let line = line.unwrap();
        let inst = Instruction::from_str(line.as_str());
        inst.unwrap()
    });

    for instruction in instructions {
        apply(&mut stack_vec, &instruction);
    }

    let mut result = String::new();
    for stack in stack_vec.array {
        if !stack.0.is_empty() {
            result.push(stack.0[stack.0.len() - 1]);
        }
    }

    result
}

fn part1(reader: impl BufRead) -> String {
    run(reader, |stack_vec, instruction| {
        let from = &mut stack_vec.array[(instruction.from - 1) as usize].0;
        let mut deque = from.split_off(from.len() - instruction.count as usize);
        deque.reverse();
        stack_vec.array[(instruction.to - 1) as usize].0.append(&mut deque);
    })
}

fn part2(reader: impl BufRead) -> String {
    run(reader, |stack_vec, instruction| {
        let from = &mut stack_vec.array[(instruction.from - 1) as usize].0;
        let mut deque = from.split_off(from.len() - instruction.count as usize);
        stack_vec.array[(instruction.to - 1) as usize].0.append(&mut deque);
    })
}

fn main() {
    let result = part1(BufReader::new(File::open("input/day5.txt").unwrap()));
    println!("part1 = {result}");
    let result = part2(BufReader::new(File::open("input/day5.txt").unwrap()));
    println!("part2 = {result}");
}

#[cfg(test)]
mod unittest {

    use super::*;

    #[test]
    fn test_example() {
        let input =
r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let reader = BufReader::new(input.as_bytes());
        assert_eq!(part1(reader), "CMZ");
        let reader = BufReader::new(input.as_bytes());
        assert_eq!(part2(reader), "MCD");
    }

}

