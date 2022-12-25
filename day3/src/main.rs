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
    let mut i: u32 = 1;
    let mut ruck1: &str; 
    for line in filetext.lines() {
        println!("{}",i);
        i += 1;
    }
}
