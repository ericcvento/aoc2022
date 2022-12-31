use std::fs; 

fn main() {
    read_data();
}

fn read_data() {
    let filetext: String = fs::read_to_string(r"data\day6input.txt").expect("Invalid File.");
    for s in filetext.chars() {
        println!("{}",s); 
    }
}
