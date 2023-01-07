use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day10input.txt").expect("Invalid File.");
    ft
}

fn process_instructions (output:String) -> i32 {

    for (i,line) in output.lines().enumerate() {
        println!("{line}"); 
    }
    1

}

fn main() {
    let input_text=read_data(); 
    process_instructions(input_text); 
}
