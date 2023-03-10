use std::fs;
use substring::Substring;

fn main() {
    part1();
    part2();
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day5input.txt").expect("Invalid File.");

    //load crates onto their starting positions
    let mut columns = vec![vec![]; 9];
    for l in filetext.lines() {
        if l.substring(1, 2) == "1" {
            break;
        }
        let mut j = 1;
        for column in columns.iter_mut() {
            let crate_content = l.substring(j, j + 1);
            if crate_content != " " {
                column.push(crate_content);
            }
            j += 4;
        }
    }

    //reverse order of crates on columns
    for i in columns.iter_mut() {
        i.reverse();
    }

    //parse crane directions
    for l in filetext.lines() {
        if l.substring(0, 4) == "move" {
            let mut n: i32 = 0;
            let mut from: usize = 0;
            let mut to: usize = 0;
            for (i, ll) in l.split_whitespace().enumerate() {
                if i == 1 {
                    n = ll.parse().unwrap();
                } else if i == 3 {
                    from = ll.parse().unwrap();
                } else if i == 5 {
                    to = ll.parse().unwrap();
                }
            }
            //execute crane instructions
            for _i in 0..n {
                let crate_contents: &str = columns[from - 1].pop().unwrap();
                columns[to - 1].push(crate_contents);
            }
        }
    }

    println!("Part 1 Solution:");
    for (i, c) in columns.iter_mut().enumerate() {
        println!("{}-{:?}", i + 1, c);
    }
}

//the ability to pick up and move multiple crates at once.
fn part2() {
    let filetext: String = fs::read_to_string(r"data\day5input.txt").expect("Invalid File.");

    //load crates onto their starting positions
    let mut columns = vec![vec![]; 9];
    for l in filetext.lines() {
        if l.substring(1, 2) == "1" {
            break;
        }
        let mut j = 1;
        for column in columns.iter_mut() {
            let crate_content = l.substring(j, j + 1);
            if crate_content != " " {
                column.push(crate_content);
            }
            j += 4;
        }
    }

    //reverse order of crates on columns
    for i in columns.iter_mut() {
        i.reverse();
    }

    //parse crane directions
    for l in filetext.lines() {
        if l.substring(0, 4) == "move" {
            let mut n: i32 = 0;
            let mut from: usize = 0;
            let mut to: usize = 0;
            for (i, ll) in l.split_whitespace().enumerate() {
                if i == 1 {
                    n = ll.parse().unwrap();
                } else if i == 3 {
                    from = ll.parse().unwrap();
                } else if i == 5 {
                    to = ll.parse().unwrap();
                }
            }
            //execute crane instructions
            let mut moving_stack: Vec<&str> = vec![];
            for _i in 0..n {
                moving_stack.push(columns[from - 1].pop().unwrap());
            }
            for _j in 0..n {
                columns[to - 1].push(moving_stack.pop().unwrap());
            }
        }
    }
    println!("Part 2 Solution");
    for (i, c) in columns.iter_mut().enumerate() {
        println!("{}-{:?}", i + 1, c);
    }
}
