use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use substring::Substring;
use thousands::Separable;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day7input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let filetext = read_data();

    //initialize cwd as root
    let mut directory_list: Vec<String> = vec![];
    let mut file_list: HashMap<String, u32> = HashMap::new();
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
            if command_prefix == "ls" {
                assert!(command_suffix.is_empty());
            }
        } else {
            //if it's not a command, it must be output
            let output_split: Vec<&str> = line.split_whitespace().collect();
            if output_split[0] == "dir" {
                directory_list.push(current_working_directory.clone() + "/" + output_split[1]);
            } else {
                file_list.insert(
                    current_working_directory.clone() + output_split[1],
                    output_split[0].parse().unwrap(),
                );
            }
        }
    }

    directory_list.sort();
    directory_list.dedup();
    //create a mapping between directory list and size of files
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    for d in directory_list.iter() {
        let mut cum_file_size: u32 = 0;
        for (k, v) in file_list.iter() {
            if d == k.substring(0, d.len()) {
                cum_file_size += v;
            }
            directory_sizes.insert(d.to_string(), cum_file_size);
        }
    }
    //size of directories under 100_000
    let mut cum_rel_dir_size: u32 = 0;
    for (_dir, size) in directory_sizes.iter() {
        if size < &100_000 {
            cum_rel_dir_size += size;
        }
    }
    println!(
        "The total size of directories under 100k is {}.",
        cum_rel_dir_size
    );

    //part2
    part2(directory_sizes, file_list);
}

fn part2(dir_sizes: HashMap<String, u32>, file_sizes: HashMap<String, u32>) -> u32 {
    //how much space do we need to clear?
    let total_disk_size = 70_000_000;
    let update_size: u32 = 30_000_000;
    let mut total_used_size: u32 = 0;
    for (_f, size) in file_sizes {
        total_used_size += size;
    }
    println!(
        "total disk size: {}",
        total_disk_size.separate_with_commas()
    );
    println!("used space: {}", total_used_size.separate_with_commas());
    println!("update size {}", update_size.separate_with_commas());

    let free_space_needed = update_size - (total_disk_size - total_used_size);

    for v in dir_sizes.values().sorted() {
        if v >= &free_space_needed {
            println!("kill the directory of size: {}", v.separate_with_commas());
            break;
        }
    }
    69
}
