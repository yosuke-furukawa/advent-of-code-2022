use regex::Regex;
use std::fs;

fn parse(arg: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let text = fs::read_to_string(arg).unwrap();
    let (cargos, moves) = text.split_once("\n\n").unwrap();
    let cargo_lines: Vec<Vec<char>> = cargos.lines().map(|line| line.chars().collect()).collect();
    let mut x = 1;
    let mut y = cargo_lines.len() as i32 - 2;
    let mut res = vec![];
    while x < cargo_lines[0].len() {
        let mut line = vec![];
        while y >= 0 {
            let c = cargo_lines[y as usize][x];
            if c != ' ' {
                line.push(c);
            }
            y -= 1;
        }
        res.push(line);
        y = cargo_lines.len() as i32 - 2;
        x += 4;
    }
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = moves
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
            )
        })
        .collect();
    (res, moves)
}

fn part1(arg: &str) -> String {
    let (cargos, moves) = parse(arg);
    let mut cargos = cargos;
    for (n, from, to) in moves {
        for _ in 0..n {
            let c = cargos[from - 1].pop().unwrap();
            cargos[to - 1].push(c);
        }
    }
    let mut res = String::new();
    for line in cargos {
        res.push(*line.last().unwrap());
    }
    res
}

fn part2(arg: &str) -> String {
    let (cargos, moves) = parse(arg);
    let mut cargos: Vec<Vec<char>> = cargos;
    for (n, from, to) in moves {
        let mut tmp = vec![];
        for _ in 0..n {
            let c = cargos[from - 1].pop().unwrap();
            tmp.push(c);
        }
        tmp.reverse();
        for c in tmp {
            cargos[to - 1].push(c);
        }
    }
    let mut res = String::new();
    for line in cargos {
        res.push(*line.last().unwrap());
    }
    res
}

fn main() {
    println!("{:?}", part1("input.txt"));
    println!("{:?}", part2("input.txt"));
}
