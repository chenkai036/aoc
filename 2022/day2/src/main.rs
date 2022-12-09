use std::fs::File;
use std::io::{BufRead, BufReader};

enum PlayerTurn {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for PlayerTurn {
    fn from(val: &str) -> Self {
        match val {
            "A" => PlayerTurn::Rock,
            "B" => PlayerTurn::Paper,
            "C" => PlayerTurn::Scissor,
            _ => panic!("Invalid hand value"),
        }
    }
}

enum MyTurn {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for MyTurn {
    fn from(val: &str) -> Self {
        match val {
            "X" => MyTurn::Rock,
            "Y" => MyTurn::Paper,
            "Z" => MyTurn::Scissor,
            _ => panic!("Invalid hand value"),
        }
    }
}

impl MyTurn {
    fn score(&self) -> u32 {
        match self {
            MyTurn::Rock => 1,
            MyTurn::Paper => 2,
            MyTurn::Scissor => 3,
        }
    }
}

enum Strategy {
    Win,
    Draw,
    Lose,
}

impl From<&str> for Strategy {
    fn from(val: &str) -> Self {
        match val {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("Invalid hand value"),
        }
    }
}

impl Strategy {
    fn score(&self) -> u32 {
        match self {
            Strategy::Win => 6,
            Strategy::Draw => 3,
            Strategy::Lose => 0,
        }
    }

    fn react(&self, player: &PlayerTurn) -> MyTurn {
        match (self, player) {
            (Strategy::Win, PlayerTurn::Rock) => MyTurn::Paper,
            (Strategy::Win, PlayerTurn::Paper) => MyTurn::Scissor,
            (Strategy::Win, PlayerTurn::Scissor) => MyTurn::Rock,
            (Strategy::Draw, PlayerTurn::Rock) => MyTurn::Rock,
            (Strategy::Draw, PlayerTurn::Paper) => MyTurn::Paper,
            (Strategy::Draw, PlayerTurn::Scissor) => MyTurn::Scissor,
            (Strategy::Lose, PlayerTurn::Rock) => MyTurn::Scissor,
            (Strategy::Lose, PlayerTurn::Paper) => MyTurn::Rock,
            (Strategy::Lose, PlayerTurn::Scissor) => MyTurn::Paper,
        }
    }
}

fn score(player: &PlayerTurn, me: &MyTurn) -> u32 {
    let outcome = match (player, me) {
        (PlayerTurn::Rock, MyTurn::Scissor) => Strategy::Lose,
        (PlayerTurn::Paper, MyTurn::Rock) => Strategy::Lose,
        (PlayerTurn::Scissor, MyTurn::Paper) => Strategy::Lose,
        (PlayerTurn::Rock, MyTurn::Rock) => Strategy::Draw,
        (PlayerTurn::Paper, MyTurn::Paper) => Strategy::Draw,
        (PlayerTurn::Scissor, MyTurn::Scissor) => Strategy::Draw,
        _ => Strategy::Win,
    };
    outcome.score() + me.score()
}

fn total_score<ScoreFn>(reader: impl BufRead, eval: ScoreFn) -> u32
where ScoreFn: Fn(&str, &str) -> u32 {
    let scores = reader.lines().map(|line| {
        let line = line.unwrap();
        let round = line.split(' ').collect::<Vec<&str>>();
        eval(round[0], round[1])
    });
    scores.sum()
}

fn part1(reader: impl BufRead) -> u32 {
    total_score(reader, |player, me| {
        let player = PlayerTurn::from(player);
        let me = MyTurn::from(me);
        score(&player, &me)
    })
}

fn part2(reader: impl BufRead) -> u32 {
    total_score(reader, |player, strategy| {
        let player = PlayerTurn::from(player);
        let strategy = Strategy::from(strategy);
        score(&player, &strategy.react(&player))
    })
}

fn main() {
    let result = part1(BufReader::new(File::open("input/day2.txt").unwrap()));
    println!("part1: {result}");
    let result = part2(BufReader::new(File::open("input/day2.txt").unwrap()));
    println!("part2: {result}");
}
