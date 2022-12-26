use std::fs;

fn main() {
    part1(); 
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day4input.txt").expect("Invalid File.");
    for l in filetext.lines() {
        let mut elf1 : &str = "";
        let mut elf2 : &str = "";
        let mut i = 1; 
        for pair in l.split(",") {
            if i == 1 {
                elf1=pair; 
            } else if i == 2 {
                elf2=pair; 
            }
            i+=1 ; 

        }
    }
}
