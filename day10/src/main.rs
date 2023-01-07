use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input.txt").expect("Invalid File.");
    ft
}

fn process_instructions(output: String) -> i32 {
    //initialize register
    let mut cycle: i32 = 0; 
    for (_i, line) in output.lines().enumerate() {
        //parse instructions
        if line.substring(0, 4) == "addx" {}
    }
    1
}

fn main() {
    let input_text = read_data();
    process_instructions(input_text);
}
