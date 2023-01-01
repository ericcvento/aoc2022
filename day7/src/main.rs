use std::fs;
use substring::Substring;

fn read_data() -> String {
    let filetext: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    return filetext;
}

fn map_directories((history): (String)) {
    let mut directory: Vec<&str> = Vec::new();
    let mut cwd: &str = "/";

    for h in history.lines() {
        //these are commands
        if h.substring(0, 1) == "$" {
            let command: &str = &h[2..4].trim();
            let command_suffix: &str = h[4..].trim();

            //checks & tests
            if command == "ls" {
                assert!(command_suffix.is_empty());
            }

            //navigate
            if command == "cd" {
                if command_suffix == ".." {
                    println!("{} {}-change to parent dir", command, command_suffix);
                } else if command_suffix == "/" {
                    println!("{} {}-back to root", command, command_suffix);
                } else {
                    assert!(!command_suffix.is_empty()); 
                    println!("{} {}-move down to {}",command, command_suffix,command_suffix); 
                }
            }
        }
    }
}

fn main() {
    let filetext = read_data();
    map_directories(filetext);
}
