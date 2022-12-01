use std::collections::BinaryHeap;
use std::fs;

fn parse(arg: &str) -> Vec<String> {
    let mut result = vec![];
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.lines();
    for line in lines {
        result.push(line.to_string());
    }
    result
}

fn part1(arg: &str) -> i32 {
    let lines = parse(arg);
    let mut max = 0;
    let mut sum = 0;
    for line in lines {
        if let Ok(v) = line.parse::<i32>() {
            sum += v;
        } else {
            max = max.max(sum);
            sum = 0;
        }
    }
    max
}

fn part2(arg: &str) -> i32 {
    let lines = parse(arg);
    let mut ranks = BinaryHeap::new();
    let mut sum = 0;
    for line in lines {
        if let Ok(v) = line.parse::<i32>() {
            sum += v;
        } else {
            ranks.push(sum);
            sum = 0;
        }
    }
    let mut res = 0;
    for _ in 0..3 {
        if let Some(v) = ranks.pop() {
            res += v;
        }
    }
    res
}

fn main() {
    println!("{}", part1("./input.txt"));
    println!("{}", part2("./input.txt"));
}
