use std::fs;
use substring::Substring;

fn main() {
    part1();
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day5input.txt").expect("Invalid File.");

    let mut columns = vec![vec![]; 9];
    for l in filetext.lines() {
        if l.substring(1, 2) == "1" {
            break;
        }
        let mut j = 1;
        for i in 0..8 {
            columns[i].push(l.substring(j, j + 1));
            j += 4;
        }
    }

    for i in 0..8 {
        println!("{:?}", columns[i]);
    }
}
