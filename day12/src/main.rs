use std::collections::HashMap;
use std::fs;

type Coordinates = (i32, i32);
type Plane = HashMap<Coordinates, char>;

fn build_grid(input_text: &str) -> (Plane,Coordinates,Coordinates) {
    let mut grid: Plane = HashMap::new();
    let mut start: Coordinates=(0,0); 
    let mut exit: Coordinates=(0,0); 
    
    let mut y = 0;
    for line in input_text.lines() {
        for (x, c) in line.chars().enumerate() {
            if c=='S' {
                start=(x as i32,y); 
            } else if c=='E' {
                exit=(x as i32,y as i32)
            }
            grid.insert((x as i32, y), c);
        }
        y -= 1;
    }
    (grid,start,exit)
}

fn return_coords(c: char, grid: Plane) -> Coordinates {
    let mut coords: Coordinates = (999, 999);
    for (k, v) in grid {
        if c == v {
            coords = k;
            break;
        }
    }
    coords
}

fn return_elevation(inputchar: char) -> u32 {
    let mut elevation: u32 = 0;
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        elevation += 1;
        if inputchar == c {
            break;
        }
    }
    elevation
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day12input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text = read_data();
    println!("{input_text}");
    let (the_grid,start,exit)= build_grid(&input_text);

    println!(
        "Starting here: {:?}, Need to exit the maze here: {:?}",
        start, exit
    );
}
