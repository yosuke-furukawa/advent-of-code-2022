use std::collections::HashSet;
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
    let mut res = 0;
    for line in lines {
        let n = line.len();
        let line = line.chars().collect::<Vec<char>>();
        let mut front = HashSet::new();
        for i in 0..n / 2 {
            front.insert(line[i]);
        }
        let mut back = HashSet::new();
        for i in n / 2..n {
            back.insert(line[i]);
        }
        front.retain(|x| back.contains(x));
        for x in front {
            let x = x as i32;
            if x >= 'a' as i32 {
                res += x - 'a' as i32 + 1;
            } else {
                res += x - 'A' as i32 + 1 + 26;
            }
        }
    }
    res
}

fn part2(arg: &str) -> i32 {
    let lines = parse(arg);
    let mut elves = vec![];
    let mut i = 0;
    while i < lines.len() {
        let mut elf = vec![];
        for j in 0..3 {
            let line = lines[i + j].clone();
            elf.push(line);
        }
        i += 3;
        elves.push(elf);
    }
    let mut res = 0;
    for elf in elves {
        let mut item1 = elf[0].chars().collect::<HashSet<char>>();
        let item2 = elf[1].chars().collect::<HashSet<char>>();
        let item3 = elf[2].chars().collect::<HashSet<char>>();
        item1.retain(|x| item2.contains(x) && item3.contains(x));
        for x in item1 {
            let x = x as i32;
            if x >= 'a' as i32 {
                res += x - 'a' as i32 + 1;
            } else {
                res += x - 'A' as i32 + 1 + 26;
            }
        }
    }
    res
}

fn main() {
    println!("{:?}", part1("./input.txt"));
    println!("{:?}", part2("./input.txt"));
}
