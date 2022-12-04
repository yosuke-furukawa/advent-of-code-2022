use std::fs;

fn parse(arg: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut result = vec![];
    let text = fs::read_to_string(arg).unwrap();
    let lines = text.lines();
    for line in lines {
        let (first, second) = line.split_once(",").unwrap();
        let ((fs, fe), (ss, se)) = (
            first.split_once("-").unwrap(),
            second.split_once("-").unwrap(),
        );
        let ((fs, fe), (ss, se)) = (
            (fs.parse::<i32>().unwrap(), fe.parse::<i32>().unwrap()),
            (ss.parse::<i32>().unwrap(), se.parse::<i32>().unwrap()),
        );
        result.push(((fs, fe), (ss, se)));
    }
    result
}

fn part1(arg: &str) -> i32 {
    let compartments = parse(arg);
    let mut res = 0;
    for ((fs, fe), (ss, se)) in compartments {
        if fs >= ss && fe <= se {
            res += 1;
        } else if ss >= fs && se <= fe {
            res += 1;
        }
    }
    res
}

fn part2(arg: &str) -> i32 {
    let compartments = parse(arg);
    let mut res = 0;
    for ((fs, fe), (ss, se)) in compartments {
        if fs <= se && fe >= ss {
            res += 1;
        }
    }
    res
}

fn main() {
    println!("{:?}", part1("./input.txt"));
    println!("{:?}", part2("./input.txt"));
}
