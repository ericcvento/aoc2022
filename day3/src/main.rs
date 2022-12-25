use std::fs;
use substring::Substring;

fn main() {
    part1();
    part2();
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day3input.txt").expect("Invalid File.");

    let mut compart1: &str;
    let mut compart2: &str;
    let mut compartsize: usize;
    let mut score: u32 = 0;

    for line in filetext.lines() {
        compartsize = line.len() / 2;
        compart1 = line.substring(0, compartsize);
        compart2 = line.substring(compartsize, line.len());
        'outer: for c1 in compart1.chars() {
            for c2 in compart2.chars() {
                if c1 == c2 {
                    let mut mapping = 1;
                    for alpha in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
                        if c1 == alpha {
                            score += mapping;
                            break 'outer;
                        }
                        mapping += 1;
                    }
                }
            }
        }
    }
    println!("Priority Sum is: {}", score);
}

fn part2() {
    let filetext: String = fs::read_to_string(r"data\day3input.txt").expect("Invalid File.");
    let mut ruck1: &str = "";
    let mut ruck2: &str = "";
    let mut ruck3: &str = "";
    let mut score: u32 = 0;
    let mut i: u32 = 1;
    for line in filetext.lines() {
        if i == 4 {
            i = 1;
        }
        if i == 1 {
            ruck1 = line;
        }
        if i == 2 {
            ruck2 = line;
        }
        if i == 3 {
            ruck3 = line;
            'outer: for r1 in ruck1.chars() {
                for r2 in ruck2.chars() {
                    if r1 == r2 {
                        for r3 in ruck3.chars() {
                            if r2 == r3 {
                                let mut mapping = 1;
                                for alpha in
                                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
                                {
                                    if r1 == alpha {
                                        score += mapping;
                                        break 'outer;
                                    }
                                    mapping += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    println!("Priority Sum is :{}", score);
}
