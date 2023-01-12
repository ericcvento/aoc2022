use std::collections::HashMap;
use std::fs;

type Coordinates = (i32, i32);
type Plane = HashMap<Coordinates, char>;

fn build_grid(input_text: &str) -> Plane {
    let mut grid: Plane = HashMap::new();
    let mut y = 0;
    for line in input_text.lines() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y), c);
        }
        y -= 1;
    }
    grid
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
    let the_grid: Plane = build_grid(&input_text);

    let origin: Coordinates = return_coords('S', the_grid.clone());
    let exit: Coordinates = return_coords('E', the_grid.clone());
    println!(
        "Starting here: {:?}, Need to exit the maze here: {:?}",
        origin, exit
    );
}
