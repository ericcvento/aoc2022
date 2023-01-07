use std::collections::HashMap;
use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input.txt").expect("Invalid File.");
    ft
}

fn read_test_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input_test.txt").expect("Invalid File.");
    ft
}

fn process_instructions(output: String) -> HashMap<i32, (i32, String)> {
    //initialize register
    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut register_history = HashMap::new();

    for (_i, line) in output.lines().enumerate() {
        //parse instructions
        let mut x = 0;
        if line.substring(0, 4) == "addx" {
            for _c in 0..2 {
                cycle += 1;
                register_history.insert(cycle, (register, line.to_string()));
            }
            x = line.substring(5, line.len()).parse::<i32>().unwrap();
        } else {
            cycle += 1;
            register_history.insert(cycle, (register, line.to_string()));
        }
        register += x;
    }
    register_history
}

fn main() {
    let input_text = read_data();
    let output_history = process_instructions(input_text);

    let mut part1_solution = 0;
    for i in 1..=output_history.len() {
        println!("{i}-{:?}", output_history[&(i as i32)]);
        if (i == 20) | ((i as i32 - 20) % 40 == 0) {
            println!("{i}");
            part1_solution += (output_history[&(i as i32)].0) * (i as i32);
        }
    }
    println!("the solution to part 1 is {part1_solution}");
}
