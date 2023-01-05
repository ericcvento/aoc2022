use std::collections::HashMap;
use std::fs;

type Coord = (i32, i32);
type CoordPlane = HashMap<Coord, u32>;

fn get_max_coord(plane: CoordPlane, axis: char) -> i32 {
    let mut max: i32 = 0;
    for kt in plane.keys() {
        if axis == 'x' && kt.0 > max {
            max = kt.0;
        }
        if axis == 'y' && kt.1 > max {
            max = kt.1;
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

fn count_hidden_trees(plane: CoordPlane) -> i32 {
    let mut hidden_tree_count = 0;
    for (coords, tree_height) in plane.clone() {
        let x = coords.0;
        let y = coords.1;

        let mut hidden_left = 0;
        let mut hidden_right = 0;
        let mut hidden_up = 0;
        let mut hidden_down = 0;

        //look 'right'
        for xx in x + 1..98 {
            if plane[&(xx, y)] >= tree_height {
                hidden_right = 1;
                break;
            }
        }
        //look 'left'
        for xx in 0..x - 1 {
            if plane[&(xx, y)] >= tree_height {
                hidden_left = 1;
                break;
            }
        }
        //look 'up'
        for yy in y + 1..98 {
            if plane[&(x, yy)] >= tree_height {
                hidden_up = 1;
                break;
            }
        }
        //look 'down'
        for yy in 0..y - 1 {
            if plane[&(x, yy)] >= tree_height {
                hidden_down = 1;
                break;
            }
        }
        let hidden_tree =
            hidden_left == 1 && hidden_right == 1 && hidden_up == 1 && hidden_down == 1;
        hidden_tree_count += hidden_tree as i32;
    }
    hidden_tree_count
}

fn main() {
    let input_text = read_data();
    let plane = create_plane(input_text);
    println!("{}", count_hidden_trees(plane));
}
