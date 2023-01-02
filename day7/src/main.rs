use std::fs;
use substring::Substring;

fn read_data() -> String {
    let filetext: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    return filetext;
}

fn main() {
    let filetext = read_data();
    //go through each line of history
    for line in filetext.lines() {
        //these are commands;
        if line.substring(0, 1) == "$" {

            //these are output
        } else {
        }
    }
}
