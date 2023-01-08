use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day11input.txt").expect("Invalid File.");
    ft
}

fn count_monkeys(input_string:&String)->i32 {
    let mut monkey_n=0; 
    for line in input_string.lines() {
        let first_word = line.split_whitespace().nth(0); 
        match first_word {
            Some(x) => {
                if x=="Monkey" {
                    monkey_n +=1; 
                }
            }
            None => {},
        }
    }
    monkey_n
}

fn main() {
    let input_string=read_data(); 
    let monkey_n = count_monkeys(&input_string); 
    println!("{monkey_n}")
}
