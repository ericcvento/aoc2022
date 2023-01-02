use std::fs;
use substring::Substring;

fn read_data() -> String {
    let filetext: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    return filetext;
}

fn map_directories(history: String) {
    let mut directory: Vec<String> = Vec::new();
    let mut cwd = String::from("/");

    for h in history.lines() {
        //these are commands
        if h.substring(0, 1) == "$" {
            let command: &str = &h[2..4].trim();
            let command_suffix: &str = h[4..].trim();

            //checks & tests
            if command == "ls" {
                assert!(command_suffix.is_empty());
                println!("{}",h); 
            }

            //navigate & build out directory structure by modifying cwd
            if command == "cd" {
                if command_suffix == "/" {
                    cwd = "/".to_string();
                } else if command_suffix == ".." {
                    let mut p: char=' '; 
                    while p != '/' {
                        p=cwd.pop().unwrap();
                    }
                    cwd.push_str("/"); 
                } else {
                    assert!(!command_suffix.is_empty());
                    cwd.push_str(command_suffix); 
                    cwd.push_str("/"); 
                }
            }
            //not really sure why I have to clone this instead of just using the value here.
            //something to do with ownership
            let dcwd=cwd.clone(); 
            directory.push(dcwd); 
        }
    }
    directory.sort();
    directory.dedup();
    for d in directory {
        //println!(""); 
        //println!("{}",d); 
    }
}

fn main() {
    let filetext = read_data();
    map_directories(filetext);
}
