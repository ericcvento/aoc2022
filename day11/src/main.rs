use std::fs;

struct Monkey {
    id: i32,
    inventory: Vec<i32>,
    operation_str: String, 
    test_divisor: i32, 
    true_monkey: i32,
    false_monkey:i32
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day11input.txt").expect("Invalid File.");
    ft
}

fn count_monkeys(input_string: &str) -> i32 {
    let mut monkey_n = 0;
    for line in input_string.lines() {
        let first_word = line.split_whitespace().nth(0);
        if first_word==Some("Monkey") {
            monkey_n += 1;
        }
    }
    monkey_n
}

fn set_starting_inventories(input_string: &str) -> Vec<Vec<i32>> {
    let mut monkeys_items=Vec::new(); 
    for line in input_string.lines() {
        let instruction = line.trim().split(':').collect::<Vec<&str>>(); 
        if instruction[0]=="Starting items" {
            let items=instruction[1].split(',').collect::<Vec<&str>>(); 
            let items_n: Vec<i32>=items.iter().map(|x| x.trim().parse::<i32>().unwrap()).collect();
            monkeys_items.push(items_n); 
        }
    }
    monkeys_items
}

fn main() {
    let input_string = read_data();
    count_monkeys(&input_string);
    let inventories=set_starting_inventories(&input_string);
}
