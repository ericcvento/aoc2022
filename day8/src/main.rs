use std::collections::HashMap;
use std::fs;

type Coord = (i32, i32);
type CoordPlane = HashMap<Coord,u32>; 

fn get_max_coord (plane:CoordPlane, axis:char) -> i32 {
    let mut max:i32=0; 
    for kt in plane.keys() {
        if axis=='x' {
            if kt.0 > max {
                max=kt.0; 
            }
        }
        if axis=='y' {
            if kt.1 > max {
                max=kt.1; 
            }
        }
    }
    max
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day8input.txt").expect("Invalid File.");
    ft
}

fn create_plane(input_text: String) -> CoordPlane {
    let mut plane = CoordPlane::new();
    for (y, line) in input_text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x as i32, y as i32);
            plane.insert(coord, c.to_digit(10).unwrap());
        }
    }
    plane
}

fn main() {
    let input_text = read_data();
    let plane = create_plane(input_text);
    
    println!("{:?}",plane); 
    println!("{}",get_max_coord(plane.clone(), 'x'));
    println!("{}",get_max_coord(plane.clone(), 'y'));
    
}
