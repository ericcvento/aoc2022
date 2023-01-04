use std::collections::HashMap;
use std::fs;

type Coord = (i32, i32);

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day8input.txt").expect("Invalid File.");
    ft
}

fn create_plane(input_text: String) -> HashMap<Coord, u32> {
    let mut plane = HashMap::new();
    for (y, line) in input_text.lines().enumerate() {
        let mut x = 0;
        for c in line.chars() {
            let coord = (x, y as i32);
            x += 1;
            plane.insert(coord, c.to_digit(10).unwrap());
        }
    }
    plane
}

fn main() {
    let input_text = read_data();
    let plane = create_plane(input_text);
}
