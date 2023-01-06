use std::fs;

type Coord = (i32, i32);

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day9input.txt").expect("Invalid File.");
    ft
}

fn push_rope(moves: String) -> i32 {
    let t_spaces = 0;
    let mut head_loc: Coord = (0, 0);
    let mut tail_loc: Coord = (0, 0);

    for l in moves.lines() {
        println!("{}", l);
    }

    t_spaces
}

fn main() {
    let input_text = read_data();
    push_rope(input_text.clone());
}
