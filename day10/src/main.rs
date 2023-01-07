use std::collections::HashMap;
use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input.txt").expect("Invalid File.");
    ft
}

fn process_instructions(output: String) -> HashMap<i32, i32> {
    //initialize register
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut register_history = HashMap::new();

    for (i, line) in output.lines().enumerate() {
        //parse instructions
        if line.substring(0, 4) == "addx" {
            for c in 0..2 {
                cycle += 1;
                if c == 1 {
                    register += line.substring(5, line.len()).parse::<i32>().unwrap();
                }
                register_history.insert(cycle, register);
            }
        } else {
            cycle += 1;
            register_history.insert(cycle, register);
        }
    }
    register_history
}

fn main() {
    let input_text = read_data();
    process_instructions(input_text);
}
