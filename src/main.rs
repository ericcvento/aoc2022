
use std::{fs};

fn main() {
    day1(); 
}

fn day1() {
    let day1filetext:String = fs::read_to_string(r"data\day1.txt")
        .expect("Invalid File.");

    for cal in day1filetext.lines(){
        if cal.is_empty() {
            println!("END"); 
        }
        else {
            let my_int: i32 = cal.parse().unwrap();
            println!("{}",my_int)
        }
    }
}

