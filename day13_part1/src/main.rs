use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    ft
}

fn convert_to_elements(input: &str) {
    println!("{input}");
}

fn main() {
    let input_text = read_data();
    for line in input_text.lines() {
        convert_to_elements(line);
    }
}
