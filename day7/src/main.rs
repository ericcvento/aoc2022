use std::{fs};
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
            let command: &str = command_split.next().unwrap().trim();
            let command_prefix: &str = command.substring(0, 2);
            let command_suffix: &str = command.substring(3, command.len());
            //change directory
            if command.substring(0, 2) == "cd" {
                println!("{} {}", command_prefix, command_suffix);
                if command_suffix=="/" {
                    current_working_directory="/".to_string(); 
                } else if command_suffix==".." {
                    let mut p : char=' '; 
                    while p != '/' {
                        p=current_working_directory.pop().unwrap(); 
                    }
                } else {
                    current_working_directory.push('/'); 
                    current_working_directory.push_str(command_suffix);
                }
            }
            println!("{}", command);
        } else {
            //these are output
        }
    }
}
