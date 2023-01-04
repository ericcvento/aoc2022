use std::collections::HashMap;
use std::fs;
use substring::Substring;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let filetext = read_data();

    //initialize cwd as root
    let mut directory_list: Vec<String> = vec![];
    let mut file_list: Vec<String> = vec![];
    let mut current_working_directory = String::from("");
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
            if command_prefix == "cd" {
                if command_suffix == "/" {
                    current_working_directory = "".to_string();
                } else if command_suffix == ".." {
                    let mut p: char = ' ';
                    while p != '/' {
                        p = current_working_directory.pop().unwrap();
                    }
                } else {
                    current_working_directory.push('/');
                    current_working_directory.push_str(command_suffix);
                }
                directory_list.push(current_working_directory.clone());
            }
            //list directory
            if command_suffix == "ls" {}
        } else {
            let output_split: Vec<&str> = line.split_whitespace().collect();
            if output_split[0] == "dir" {
                directory_list.push(current_working_directory.clone() + "/" + output_split[1]);
            } else {
                file_list.push(current_working_directory.clone() + "/" + output_split[0]);
            }
        }
    }
    directory_list.sort();
    directory_list.dedup();

    for d in directory_list.iter() {
        println!("{d}");
    }
    for f in file_list.iter() {
        println!("{f}");
    }
}
