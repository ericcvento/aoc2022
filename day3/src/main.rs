use std::fs;

fn main() {
    part1();
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day3input.txt").expect("Invalid File.");
}
