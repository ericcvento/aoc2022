use std::fs;

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day11input.txt").expect("Invalid File.");
    ft
}

fn count_monkeys(input_string: &String) -> i32 {
    let mut monkey_n = 0;
    for line in input_string.lines() {
        let first_word = line.split_whitespace().nth(0);
        if first_word==Some("Monkey") {
            monkey_n += 1;
        }
    }
    monkey_n
}

fn set_starting_inventories(input_string: &String) {
    for line in input_string.lines() {
        let mut instruction = line.trim().split(':').collect::<Vec<&str>>(); 
        //instruction.retain(|&x| x != ""); 
        if instruction[0]=="Starting items" {
            println!("{:?}",instruction);
        }
    }

}

fn main() {
    let input_string = read_data();
    let monkey_n = count_monkeys(&input_string);
    set_starting_inventories(&input_string);
}
