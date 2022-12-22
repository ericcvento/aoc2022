use std::{fs};
use substring::Substring;

fn main() {
    println!("Hello, world!");

    let filetext: String = fs::read_to_string(r"data\day2input.txt")
        .expect("Invalid File.");

    for line in filetext.lines() {

        let myplay:&str=(line.substring(0,1)); 
        let yourplay:&str = line.substring(2,3); 
    }
}