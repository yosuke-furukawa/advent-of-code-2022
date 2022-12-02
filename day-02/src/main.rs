use std::fs;

fn parse1(arg: &str) -> Vec<(&str, &str)> {
    let mut result = vec![];
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.lines();
    for line in lines {
        let splits: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let enemy = match splits[0] {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            _ => unreachable!(),
        };
        let player = match splits[1] {
            "X" => "Rock",
            "Y" => "Paper",
            "Z" => "Scissors",
            _ => unreachable!(),
        };
        result.push((enemy, player));
    }
    result
}

fn parse2(arg: &str) -> Vec<(&str, &str)> {
    let mut result = vec![];
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.lines();
    for line in lines {
        let splits: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let enemy = match splits[0] {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            _ => unreachable!(),
        };
        let player = match splits[1] {
            "X" => "Lose",
            "Y" => "Draw",
            "Z" => "Win",
            _ => unreachable!(),
        };
        result.push((enemy, player));
    }
    result
}

fn part1(arg: &str) -> i32 {
    let rock_paper_scissors = parse1(arg);
    let mut sum = 0;
    for (a, b) in rock_paper_scissors {
        sum += match (a, b) {
            ("Rock", "Rock") => 1 + 3,
            ("Rock", "Paper") => 2 + 6,
            ("Rock", "Scissors") => 3 + 0,
            ("Paper", "Rock") => 1 + 0,
            ("Paper", "Paper") => 2 + 3,
            ("Paper", "Scissors") => 3 + 6,
            ("Scissors", "Rock") => 1 + 6,
            ("Scissors", "Paper") => 2 + 0,
            ("Scissors", "Scissors") => 3 + 3,
            _ => unreachable!(),
        };
    }
    sum
}

fn part2(arg: &str) -> i32 {
    let rock_paper_scissors = parse2(arg);
    let mut sum = 0;
    for (a, b) in rock_paper_scissors {
        sum += match (a, b) {
            ("Rock", "Lose") => 3 + 0,
            ("Rock", "Draw") => 1 + 3,
            ("Rock", "Win") => 2 + 6,
            ("Paper", "Lose") => 1 + 0,
            ("Paper", "Draw") => 2 + 3,
            ("Paper", "Win") => 3 + 6,
            ("Scissors", "Lose") => 2 + 0,
            ("Scissors", "Draw") => 3 + 3,
            ("Scissors", "Win") => 1 + 6,
            _ => unreachable!(),
        };
    }
    sum
}

fn main() {
    println!("{}", part1("./input.txt"));
    println!("{}", part2("./input.txt"));
}
