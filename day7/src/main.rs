use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let filetext = read_data();

    //initialize cwd as root
    let mut current_working_directory = String::from("/");
    //go through each line of history
    for line in filetext.lines() {
        if line.substring(0, 1) == "$" {
            //these are commands;
            let mut command_split = line.split('$');
            command_split.next();
            let command: &str = command_split.next().unwrap();
            println!("{}", command);
        } else {
            //these are output
        }
    }
}
