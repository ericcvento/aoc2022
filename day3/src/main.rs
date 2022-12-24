use std::fs;
use substring::Substring;

fn main() {
    part1();
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day3input.txt").expect("Invalid File.");

    let mut compart1: &str = "";
    let mut compart2: &str = "";
    let mut compartsize: usize = 0;

    for line in filetext.lines() {
        compartsize = line.len() / 2;
        compart1 = line.substring(0, compartsize);
        compart2 = line.substring(compartsize, line.len());

        println!("Ruck: {}, Compartment Size: {}", line, compartsize);
        println!("Compartment1:{}, Compartment2:{}", compart1, compart2);
    }
}
