use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day9input.txt").expect("Invalid File.");
    ft
}

fn push_rope(moves: String) -> i32 {
    let tail_spaces = 0;
    let mut head_loc: [i32;2] = [0,0];
    let mut tail_loc: [i32;2] = [0,0];

    for (i,l) in moves.lines().enumerate() {
        let instructions: Vec<&str> = l.split_whitespace().collect();
        let direction = instructions[0];
        let moves: i32 = instructions[1].parse().unwrap();

        let mut position_change: [i32;2] = [0,0];
        for m in 0..moves {
            if direction == "U" {
                position_change=[0,1]
            } else if direction == "D" {
                position_change=[0,-1]
            } else if direction == "L" {
                position_change=[-1,0]
            } else if direction == "R" {
                position_change=[1,0]
            }
            head_loc=[head_loc[0]+position_change[0],head_loc[1]+position_change[1]]; 
        }
    }
    tail_spaces
}

fn main() {
    let input_text = read_data();
    push_rope(input_text.clone());
}
