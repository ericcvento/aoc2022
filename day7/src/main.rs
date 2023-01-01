use std::fs; 

fn read_data() -> String {
    let filetext:String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    return filetext; 
}

fn main() {
    let filetext=read_data();
    for l in filetext.lines() {
        println!("{}",l); 
    }
}