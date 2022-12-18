
use std::{fs};

fn main() {
    day1(); 
}

fn day1() {
    let day1filetext:String = fs::read_to_string(r"data\day1.txt")
        .expect("Invalid File.");

    println!("{}",day1filetext);
}