use std::fs;

fn main() {
    part1(); 
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day4input.txt").expect("Invalid File.");
    for elfpair in filetext.lines() {
   
        let (fst,snd) = elfpair.split_once(',').unwrap();
    }
}
