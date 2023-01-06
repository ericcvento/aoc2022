use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day9input.txt").expect("Invalid File.");
    ft
}

fn push_rope_two_knots(moves: String) -> i32 {
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

fn push_rope_10_knots(moves: String) -> i32 {
    let mut knot_ten_history = Vec::new();

    //load vec with the starting location of 10 knots.
    let mut knots = vec![];
    for _i in 0..10 {
        knots.push([0, 0])
    }

    //....
    for (i, l) in moves.lines().enumerate() {
        let instructions: Vec<&str> = l.split_whitespace().collect();
        let direction = instructions[0];
        let moves: i32 = instructions[1].parse().unwrap();

        if i < 1000 {
            println!("{i}-{direction}-{moves}-{:?}", knots);
        }

        for _m in 0..moves {
            for k in [1,2,3,4,5,6,7,8,9,0] {
                if k > 0 {
                    let diff: [i32; 2] =
                        [knots[k - 1][0] - knots[k][0], knots[k - 1][1] - knots[k][1]];
                    let mut tail_correction = [0, 0];
                    if diff[0].abs() + diff[1].abs() == 3 {
                        for d in 0..2 {
                            if diff[d] > 0 {
                                tail_correction[d] = 1
                            }
                            if diff[d] < 0 {
                                tail_correction[d] = -1
                            }
                        }
                    } else if diff[0].abs() + diff[1].abs() == 2 {
                        for d in 0..2 {
                            if diff[d] == 2 {
                                tail_correction[d] = 1
                            }
                            if diff[d] == -2 {
                                tail_correction[d] = -1
                            }
                        }
                    }

                    knots[k] = [
                        knots[k][0] + tail_correction[0],
                        knots[k][1] + tail_correction[1],
                    ];

                    if k == 9 {
                        knot_ten_history.push(knots[k]);
                        //println!("{:?}",knot_ten_history)
                    }

                } else if k == 0 {
                    //move head
                    let mut position_change: [i32; 2] = [0, 0];
                    if direction == "U" {
                        position_change = [0, 1]
                    } else if direction == "D" {
                        position_change = [0, -1]
                    } else if direction == "L" {
                        position_change = [-1, 0]
                    } else if direction == "R" {
                        position_change = [1, 0]
                    }
                    knots[0] = [
                        knots[0][0] + position_change[0],
                        knots[0][1] + position_change[1],
                    ];
                }
            }
        }
    }
    knot_ten_history.sort();
    knot_ten_history.dedup();
    knot_ten_history.len() as i32
}

fn main() {
    let input_text = read_data();
    let p1_solution = push_rope_two_knots(input_text.clone());
    println!("the solution to part one is {p1_solution}");

    let p2_solution = push_rope_10_knots(input_text);
    println!("the solution to part two is {p2_solution}");
}
