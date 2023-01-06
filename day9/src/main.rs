use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day9input.txt").expect("Invalid File.");
    ft
}

fn push_rope(moves: String) -> i32 {
    let mut head_loc: [i32; 2] = [0, 0];
    let mut tail_loc: [i32; 2] = [0, 0];
    let mut tail_loc_history = Vec::new();

    for (_i, l) in moves.lines().enumerate() {
        let instructions: Vec<&str> = l.split_whitespace().collect();
        let direction = instructions[0];
        let moves: i32 = instructions[1].parse().unwrap();

        //where is tail?
        //move tail if necessary
        //move head
        let mut position_change: [i32; 2] = [0, 0];
        for _m in 0..moves {
            let head_tail_diff = [head_loc[0] - tail_loc[0], head_loc[1] - tail_loc[1]];

            let mut tail_correction = [0, 0];
            if head_tail_diff[0].abs() + head_tail_diff[1].abs() == 3 {
                for d in 0..2 {
                    if head_tail_diff[d] > 0 {
                        tail_correction[d] = 1
                    }
                    if head_tail_diff[d] < 0 {
                        tail_correction[d] = -1
                    }
                }
            } else if head_tail_diff[0].abs() + head_tail_diff[1].abs() == 2 {
                for d in 0..2 {
                    if head_tail_diff[d] == 2 {
                        tail_correction[d] = 1
                    }
                    if head_tail_diff[d] == -2 {
                        tail_correction[d] = -1
                    }
                }
            }

            tail_loc = [
                tail_loc[0] + tail_correction[0],
                tail_loc[1] + tail_correction[1],
            ];

            //track tail history
            tail_loc_history.push(tail_loc);

            if direction == "U" {
                position_change = [0, 1]
            } else if direction == "D" {
                position_change = [0, -1]
            } else if direction == "L" {
                position_change = [-1, 0]
            } else if direction == "R" {
                position_change = [1, 0]
            }
            head_loc = [
                head_loc[0] + position_change[0],
                head_loc[1] + position_change[1],
            ];
        }
    }
    tail_loc_history.sort();
    tail_loc_history.dedup();
    tail_loc_history.len() as i32
}

fn main() {
    let input_text = read_data();
    let p1_solution = push_rope(input_text);
    println!("the solution to part one is {p1_solution}");
}
