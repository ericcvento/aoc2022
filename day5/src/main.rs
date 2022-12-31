use std::fs;
use substring::Substring; 

fn main() {
    part1(); 
}

fn part1(){
    let filetext: String = fs::read_to_string(r"data\day5input.txt").expect("Invalid File.");
    for l in filetext.lines() {

        let c1 = l.substring(1,2); 
        let c2 = l.substring(5,6); 
        let c3 = l.substring(9,10); 
        let c4 = l.substring(13,14); 
        let c5 = l.substring(17,18); 
        let c6 = l.substring(21,21); 
        let c7 = l.substring(25,26); 
        let c8 = l.substring(29,30); 
        let c9 = l.substring(33,34); 
        
        if c1 == "1" {
            break; 
        }
    }
}