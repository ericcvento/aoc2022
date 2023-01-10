use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day12input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text=read_data(); 
    println!("{input_text}");
}
