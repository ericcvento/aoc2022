use std::fs;
fn main() {
    println!("Hello, world!");

    let filetext: String = fs::read_to_string(r"data\day2input.txt")
        .expect("Invalid File.");

    for l in filetext.lines() {
        println!("{}",l); 
    }
}