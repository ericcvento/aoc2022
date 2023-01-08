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

fn count_visible_trees(plane: CoordPlane) -> i32 {
    let mut visible_tree_count = 0;
    for (coords, tree_height) in plane.clone() {
        let x = coords.0;
        let y = coords.1;

        let mut hidden_left = 0;
        let mut hidden_right = 0;
        let mut hidden_up = 0;
        let mut hidden_down = 0;

        //look 'right'
        for xx in x + 1..=98 {
            if plane[&(xx, y)] >= tree_height {
                hidden_right = 1;
                break;
            }
        }
        //look 'left'
        for xx in 0..x {
            if plane[&(xx, y)] >= tree_height {
                hidden_left = 1;
                break;
            }
        }
        //look 'up'
        for yy in y + 1..=98 {
            if plane[&(x, yy)] >= tree_height {
                hidden_up = 1;
                break;
            }
        }
        //look 'down'
        for yy in 0..y {
            if plane[&(x, yy)] >= tree_height {
                hidden_down = 1;
                break;
            }
        }
        let visible_tree = hidden_left + hidden_right + hidden_up + hidden_down < 4;
        visible_tree_count += visible_tree as i32;
    }
    visible_tree_count
}

fn find_max_scenic_score(plane: CoordPlane) -> i32 {
    let mut max_scenic_score = 0;
    for (coords, tree_height) in plane.clone() {
        let x = coords.0;
        let y = coords.1;

        let mut trees_left = 0;
        let mut trees_right = 0;
        let mut trees_up = 0;
        let mut trees_down = 0;

        //look 'right'
        for xx in x + 1..=98 {
            trees_right += 1;
            if plane[&(xx, y)] >= tree_height {
                break;
            }
        }
        //look 'left'
        for xx in (0..x).rev() {
            trees_left += 1;
            if plane[&(xx, y)] >= tree_height {
                break;
            }
        }
        //look 'up'
        for yy in y + 1..=98 {
            trees_up += 1;
            if plane[&(x, yy)] >= tree_height {
                break;
            }
        }
        //look 'down'
        for yy in (0..y).rev() {
            trees_down += 1;
            if plane[&(x, yy)] >= tree_height {
                break;
            }
        }

        let scenic_score = trees_left * trees_right * trees_up * trees_down;
        if scenic_score > max_scenic_score {
            max_scenic_score = scenic_score;
        }
    }
    max_scenic_score
}

fn main() {
    let input_text = read_data();
    let plane = create_plane(input_text);
    println!(
        "number of visible trees:{}",
        count_visible_trees(plane.clone())
    );
    //part 2
    println!("max scenic score:{}", find_max_scenic_score(plane));
}
