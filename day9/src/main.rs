use std::fs; 

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day8input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text=read_data(); 
}
