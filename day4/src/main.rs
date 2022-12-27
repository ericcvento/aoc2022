use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut score: i32 = 0;
    let filetext: String = fs::read_to_string(r"data\day4input.txt").expect("Invalid File.");
    for elfpair in filetext.lines() {
        let (elf1, elf2) = elfpair.split_once(',').unwrap();
        let (elf1start, elf1end) = elf1.split_once('-').unwrap();
        let (elf2start, elf2end) = elf2.split_once('-').unwrap();

        //is this OK to do?
        let elf1start: i32 = elf1start.parse().unwrap();
        let elf1end: i32 = elf1end.parse().unwrap();
        let elf2start: i32 = elf2start.parse().unwrap();
        let elf2end: i32 = elf2end.parse().unwrap();

        let mut subsumed: i32 = 0;
        if (elf1start <= elf2start) & (elf1end >= elf2end) {
            subsumed = 1;
        }
        if (elf2start <= elf1start) & (elf2end >= elf1end) {
            subsumed = 1;
        }
        score += subsumed;
    }
    println!("{}", score);
}

fn part2() {
    let mut score: i32 = 0;
    let filetext: String = fs::read_to_string(r"data\day4input.txt").expect("Invalid File.");
    for elfpair in filetext.lines() {
        let (elf1, elf2) = elfpair.split_once(',').unwrap();
        let (elf1start, elf1end) = elf1.split_once('-').unwrap();
        let (elf2start, elf2end) = elf2.split_once('-').unwrap();

        //is this OK to do?
        let elf1start: i32 = elf1start.parse().unwrap();
        let elf1end: i32 = elf1end.parse().unwrap();
        let elf2start: i32 = elf2start.parse().unwrap();
        let elf2end: i32 = elf2end.parse().unwrap();

        let mut subsumed: i32 = 0;
        if (elf1start <= elf2start) & (elf1end >= elf2start) {
            subsumed = 1;
        }
        if (elf1start <= elf2end) & (elf1end >= elf2end) {
            subsumed = 1;
        }
        if (elf2start <= elf1start) & (elf2end >= elf1start) {
            subsumed = 1;
        }
        if (elf2start <= elf1end) & (elf2end >= elf1end) {
            subsumed = 1;
        }
        score += subsumed;
    }
    println!("{}", score);
}
