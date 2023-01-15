use std::collections::HashMap;
use std::fs;

type Coordinates = (i32, i32);
type Plane = HashMap<Coordinates, char>;
type FourCoordinates = [Coordinates; 4];

fn build_grid(input_text: &str) -> (Plane, Coordinates, Coordinates) {
    let mut grid: Plane = HashMap::new();
    let mut start: Coordinates = (0, 0);
    let mut exit: Coordinates = (0, 0);

    let mut y = 0;
    for line in input_text.lines() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y);
                grid.insert((x as i32, y), 'a');
            } else if c == 'E' {
                exit = (x as i32, y);
                grid.insert((x as i32, y), 'z');
            } else {
                grid.insert((x as i32, y), c);
            }
        }
        y -= 1;
    }
    (grid, start, exit)
}

fn convert_to_elevation(inputchar: char) -> i32 {
    let mut elevation: i32 = 0;
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        elevation += 1;
        if inputchar == c {
            break;
        }
    }
    elevation
}

fn get_elevation(loc: Coordinates, map: &Plane) -> i32 {
    convert_to_elevation(map[&loc])
}

fn return_neighbors(loc: &Coordinates) -> FourCoordinates {
    //left, right, up, down
    [
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 + 1),
        (loc.0, loc.1 - 1),
    ]
}

fn check_neighbors(map: &Plane, neighbors: FourCoordinates) -> HashMap<char, (Coordinates, i32)> {
    let mut neighbor_details: HashMap<char, (Coordinates, i32)> = HashMap::new();

    let mut i = 0;
    for c in neighbors {
        if map.contains_key(&c) {
            let direction = match i {
                0 => 'L',
                1 => 'R',
                2 => 'U',
                3 => 'D',
                _ => ' ',
            };
            neighbor_details.insert(direction, (c, convert_to_elevation(map[&c])));
        }
        i += 1;
    }
    neighbor_details
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day12input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text = read_data();
    println!("{input_text}");
    let (the_grid, start, exit) = build_grid(&input_text);
    println!(
        "Starting here: {:?}, Need to exit the maze here: {:?}",
        start, exit
    );

    let mut routes: Vec<Vec<Coordinates>> = Vec::new();
    routes.push(vec![start]);
    let mut routes_n = routes.len();

    let mut current_route = Vec::new();

    for maini in 0..100 {
        let mut add_routes = Vec::new();
        for i in 0..routes_n {
            add_routes = Vec::new();
            current_route = routes.pop().unwrap();
            let current_loc = current_route.pop().unwrap();
            let neighbors = return_neighbors(&current_loc);
            let neighbors = check_neighbors(&the_grid, neighbors);
            'look: for k in neighbors.keys() {
                if neighbors[k].1 - get_elevation(current_loc.clone(), &the_grid) <= 1 {
                    let add_loc = neighbors[k].0;
                    if Some(add_loc) == current_route.last().copied() {
                        continue 'look;
                    }
                    current_route.push(current_loc);
                    current_route.push(add_loc);
                    add_routes.push(current_route.clone());
                    current_route.pop();
                    current_route.pop();
                }
            }
        }
        routes.extend(add_routes);
        routes_n = routes.len();
        println!("{maini}-{routes_n}");
        for r in &routes {
            println!("{:?}", r);
        }
    }
}
