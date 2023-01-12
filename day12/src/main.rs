use std::fs;
use std::collections::HashMap; 

struct Coordinates {
    x:i32,
    y:i32
}
fn load_Coordinates(x:i32,y:i32) -> Coordinates {
    Coordinates {x,y}
}

fn build_grid(input_text:&str) -> HashMap<Coordinates,char> {
    let grid = HashMap::new(); 
    let mut y = 0;
    for line in input_text.lines() {
        let mut x = 0; 
        for c in line.chars() {
            
        }
    }
    grid
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day12input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text=read_data(); 
    println!("{input_text}");
    build_grid(&input_text); 
}
