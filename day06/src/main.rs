use std::fs;

fn main() {
    part1();
    part2(); 
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day6input.txt").expect("Invalid File.");
    let mut four = Vec::new();
    for (i, s) in filetext.chars().enumerate() {
        four.push(s);
        if four.len() >= 4 {
            let mut four_copy = four.to_vec();
            four_copy.sort();
            four_copy.dedup();
            if four_copy.len() == 4 {
                println!("Part 1 Solution: {}", i + 1);
                break;
            }
            four.reverse();
            four.pop();
            four.reverse();
        }
    }
}

fn part2() {
    let filetext: String = fs::read_to_string(r"data\day6input.txt").expect("Invalid File.");
    let mut four = Vec::new();
    for (i, s) in filetext.chars().enumerate() {
        four.push(s);
        if four.len() >= 14 {
            let mut four_copy = four.to_vec();
            four_copy.sort();
            four_copy.dedup();
            if four_copy.len() == 14 {
                println!("Part 2 Solution: {}", i + 1);
                break;
            }
            four.reverse();
            four.pop();
            four.reverse();
        }
    }
}
