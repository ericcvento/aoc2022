use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input.txt").expect("Invalid File.");
    ft
}

fn process_instructions(output: String) -> i32 {
    //initialize register
    let mut cycles: i32 = 0;
    let mut register: i32 = 1;
    let mut x: i32 = 0;
    for (_i, line) in output.lines().enumerate() {
        //parse instructions
        if line.substring(0, 4) == "addx" {
            //need to output register at certain cycles, add loop that takes 2 steps here; 
            cycles += 2;
            x += line.substring(5, line.len()).parse::<i32>().unwrap();
        }
        if cycles 
    }
    1
}

fn main() {
    let input_text = read_data();
    process_instructions(input_text);
}
