use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn parse(arg: &str) -> (HashMap<String, i32>, HashMap<String, i32>) {
    let mut files = HashMap::new();
    let mut dirs = HashMap::new();
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.lines();
    let mut cwd = vec![];
    let cd = Regex::new(r"\$ cd (.+)").unwrap();
    let ls = Regex::new(r"\$ ls").unwrap();
    let file = Regex::new(r"(\d+) (.+)").unwrap();
    let dir = Regex::new(r"dir (.+)").unwrap();

    for line in lines {
        match line {
            line if cd.is_match(line) => {
                let caps = cd.captures(line).unwrap();
                if &caps[1] == ".." {
                    cwd.pop();
                    continue;
                }
                cwd.push(caps[1].to_string());
            }
            line if ls.is_match(line) => {
                // noop
            }
            line if file.is_match(line) => {
                let caps = file.captures(line).unwrap();
                let size = caps[1].parse::<i32>().unwrap();
                let name = caps[2].to_string();
                let dir = cwd.join("/").to_string();
                let f = dir.clone() + "/" + &name;
                if !files.contains_key(&f) {
                    files.insert(f, size);
                    let mut tmp = "".to_string();
                    for d in &cwd {
                        tmp += &(d.to_owned() + "/");
                        *dirs.entry(tmp.clone()).or_insert(0) += size;
                    }
                }
            }
            line if dir.is_match(line) => {
                // noop
            }
            _ => {}
        }
    }
    (files, dirs)
}

fn part1(arg: &str) -> i32 {
    let (_, dirs) = parse(arg);
    dirs.values().filter(|&x| x <= &100000).sum::<i32>()
}

fn part2(arg: &str) -> i32 {
    let (_, dirs) = parse(arg);
    let mut entries: Vec<_> = dirs.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    let unused = 70_000_000 - entries[0].1;
    let mut res = 0;
    for entry in entries.iter().skip(1) {
        if unused + entry.1 >= 30_000_000 {
            res = *entry.1;
        } else {
            break;
        }
    }
    res
}

fn main() {
    println!("{}", part1("./input.txt"));
    println!("{}", part2("./input.txt"));
}
