use std::fs;
use std::collections::HashMap; 

type Coordinates = (i32, i32);

fn build_grid(input_text:&str) -> HashMap<Coordinates,char> {
    let mut grid = HashMap::new(); 
    let mut y = 0;
    for line in input_text.lines() {
        let mut x = 0; 
        for c in line.chars() {
            grid.insert((x,y),c);
            x+=1; 
        }
        y-=1; 
    }
    grid
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day12input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_text=read_data(); 
    println!("{input_text}");
    let the_grid=build_grid(&input_text);
    for (k,v) in the_grid {
        if v=='S' {
            println!("{:?}",k);
        } 
    }
}
