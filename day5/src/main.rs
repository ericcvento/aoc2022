use std::fs;
use substring::Substring;

fn main() {
    part1();
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
            column.push(l.substring(j, j + 1));
            j += 4;
        }
    }

    //reverse order of crates on columns
    for i in columns.iter_mut() {
        println!("{:?}",i); 
        i.reverse();
        println!("{:?}",i);
    }

    //crane directions
    for l in filetext.lines() {
        if l.substring(0, 4) == "move" {
            let mut n : i32=0; 
            let mut from :i32=0; 
            let mut to: i32=0;             
            for (i,ll)  in l.split_whitespace().enumerate() {
                if i==1 {
                    n = ll.parse().unwrap();
                } else if i == 3 {
                    from = ll.parse().unwrap();
                } else if i == 5 {
                    to = ll.parse().unwrap();
                }
            }
        //execute instructions; 
        
        }
    }
}