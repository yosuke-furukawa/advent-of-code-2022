use std::fs;

fn parse(arg: &str) -> Vec<Vec<u32>> {
    let text = fs::read_to_string(arg).unwrap();
    let mut res = vec![];
    for line in text.lines() {
        let trees: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        res.push(trees);
    }
    res
}

fn part1(arg: &str) -> i32 {
    let map = parse(arg);
    let h = map.len();
    let w = map[0].len();
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = map[y][x];
            let mut visible_top = true;
            let mut visible_bottom = true;
            let mut visible_left = true;
            let mut visible_right = true;
            for dx in 0..x {
                if dx != x && map[y][dx] >= cell {
                    visible_top = false;
                    break;
                }
            }
            for dx in x + 1..w {
                if dx != x && map[y][dx] >= cell {
                    visible_bottom = false;
                    break;
                }
            }
            for dy in 0..y {
                if dy != y && map[dy][x] >= cell {
                    visible_left = false;
                    break;
                }
            }
            for dy in y + 1..h {
                if dy != y && map[dy][x] >= cell {
                    visible_right = false;
                    break;
                }
            }
            if visible_top || visible_bottom || visible_left || visible_right {
                count += 1;
            }
        }
    }
    count
}

fn part2(arg: &str) -> i32 {
    let map = parse(arg);
    let h = map.len();
    let w = map[0].len();
    let mut scenic_score_max = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = map[y][x];
            let mut visible_top = 0;
            let mut visible_bottom = 0;
            let mut visible_left = 0;
            let mut visible_right = 0;
            for dx in (0..x).rev() {
                visible_top += 1;
                if map[y][dx] >= cell {
                    break;
                }
            }
            for dx in x + 1..w {
                visible_bottom += 1;
                if map[y][dx] >= cell {
                    break;
                }
            }
            for dy in (0..y).rev() {
                visible_left += 1;
                if map[dy][x] >= cell {
                    break;
                }
            }
            for dy in y + 1..h {
                visible_right += 1;
                if map[dy][x] >= cell {
                    break;
                }
            }
            let scenic_score = visible_top * visible_bottom * visible_left * visible_right;
            if scenic_score > scenic_score_max {
                scenic_score_max = scenic_score;
            }
        }
    }
    scenic_score_max
}

fn main() {
    println!("{:?}", part1("input.txt"));
    println!("{:?}", part2("input.txt"));
}
