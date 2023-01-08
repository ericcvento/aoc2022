use std::fs;

struct Monkey {
    id: i32,
    inventory: Vec<i32>,
    operation_str: String,
    test_divisor: i32,
    true_monkey: i32,
    false_monkey: i32,
}

fn build_monkey(id: i32, inventory: Vec<i32>) -> Monkey {
    Monkey {
        id,
        inventory,
        operation_str: "".to_string(),
        test_divisor: 0,
        true_monkey: 99,
        false_monkey: 99,
    }
}

fn set_starting_inventories(input_string: &str) -> Vec<Vec<i32>> {
    let mut monkeys_items = Vec::new();
    for line in input_string.lines() {
        let instruction = line.trim().split(':').collect::<Vec<&str>>();
        if instruction[0] == "Starting items" {
            let items = instruction[1].split(',').collect::<Vec<&str>>();
            let items_n: Vec<i32> = items
                .iter()
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect();
            monkeys_items.push(items_n);
        }
    }
    monkeys_items
}

fn parse_ops(input_string: &str) -> Vec<String> {
    let mut ops = Vec::new();
    for line in input_string.lines() {
        let instruction = line.trim().split(':').collect::<Vec<&str>>();
        if instruction[0] == "Operation" {
            ops.push(instruction[1].trim().to_string())
        }
    }
    ops
}

fn parse_divisors(input_string: &str) -> Vec<i32> {
    let mut divisors = Vec::new();
    for line in input_string.lines() {
        let instruction = line.trim().split(':').collect::<Vec<&str>>();
        if instruction[0] == "Test" {
            let t: String = instruction[1].chars().filter(|c| c.is_digit(10)).collect();
            let divisor = t.parse::<i32>().unwrap();
            divisors.push(divisor); 
        }
    }
    divisors
}

fn count_monkeys(input_string: &str) -> i32 {
    let mut monkey_n = 0;
    for line in input_string.lines() {
        let first_word = line.split_whitespace().nth(0);
        if first_word == Some("Monkey") {
            monkey_n += 1;
        }
    }
    monkey_n
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day11input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_string = read_data();
    let monkey_count = count_monkeys(&input_string);
    let inventories = set_starting_inventories(&input_string);
    parse_ops(&input_string);
    parse_divisors(&input_string);
    

    for i in 0..monkey_count {}
}
