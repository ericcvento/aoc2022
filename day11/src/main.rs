use std::fs;

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    inventory: Vec<i32>,
    operation_str: String,
    test_divisor: i32,
    tfmonkeys: (usize, usize),
    monkey_biz: i32,
}

fn build_monkey(
    id: usize,
    inventory: Vec<i32>,
    operation_str: String,
    test_divisor: i32,
    tfmonkeys: (usize, usize),
) -> Monkey {
    Monkey {
        id,
        inventory,
        operation_str,
        test_divisor,
        tfmonkeys,
        monkey_biz: 0,
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
            let t: String = instruction[1]
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            let divisor = t.parse::<i32>().unwrap();
            divisors.push(divisor);
        }
    }
    divisors
}

fn parse_true_false_monkeys(input_string: &str) -> Vec<(usize, usize)> {
    let mut tfmonkeys = Vec::new();
    let mut tfmonkey: (usize, usize) = (0, 0);
    for line in input_string.lines() {
        let instruction = line.trim().split(':').collect::<Vec<&str>>();

        if instruction[0] == "If true" || instruction[0] == "If false" {
            let idnumber: String = instruction[1]
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            let idnumber: usize = idnumber.parse().unwrap();
            if instruction[0] == "If true" {
                tfmonkey.0 = idnumber;
            } else if instruction[0] == "If false" {
                tfmonkey.1 = idnumber;
                tfmonkeys.push(tfmonkey);
                tfmonkey = (0, 0);
            }
        }
    }
    tfmonkeys
}

fn count_monkeys(input_string: &str) -> i32 {
    let mut monkey_n = 0;
    for line in input_string.lines() {
        let first_word = line.split_whitespace().next();
        if first_word == Some("Monkey") {
            monkey_n += 1;
        }
    }
    monkey_n
}

fn game_loop(mut monkeys: Vec<Monkey>) -> i32 {
    for _round in 1..=20 {
        for m in 0..monkeys.len() {
            monkeys[m].inventory.reverse();
            //test worry, parsing this out in a ridiculous way
            let monkeys_clone = monkeys.clone();
            let ops: Vec<&str> = monkeys_clone[m].operation_str.split_whitespace().collect();
            assert!(ops.len() == 5);
            assert!(ops[3] == "*" || ops[3] == "+");
            for _i in 0..monkeys[m].inventory.len() {
                let old = monkeys[m].inventory.pop().unwrap();
                let comparator: i32 = if ops[4] == "old" {
                    old
                } else {
                    ops[4].parse::<i32>().unwrap()
                };
                let new: i32 = if ops[3] == "*" {
                    (old * comparator) / 3
                } else {
                    (old + comparator) / 3
                };
                let relmonkey: usize = if (new % monkeys[m].test_divisor) == 0 {
                    monkeys[m].tfmonkeys.0
                } else {
                    monkeys[m].tfmonkeys.1
                };
                monkeys[relmonkey].inventory.push(new);
                monkeys[m].monkey_biz += 1;
            }
        }
    }
    let mut monkey_biz_v: Vec<i32> = Vec::new();
    for m in monkeys {
        monkey_biz_v.push(m.monkey_biz);
    }
    monkey_biz_v.sort();
    monkey_biz_v.reverse();
    monkey_biz_v[0] * monkey_biz_v[1]
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day11input.txt").expect("Invalid File.");
    ft
}

fn main() {
    let input_string = read_data();
    let monkey_count = count_monkeys(&input_string) as usize;
    let inventories = set_starting_inventories(&input_string);
    let ops = parse_ops(&input_string);
    let divisors = parse_divisors(&input_string);
    let tfmonkeys = parse_true_false_monkeys(&input_string);

    //load monkeys in to a vec
    let mut monkeys: Vec<Monkey> = Vec::new();
    for i in 0..monkey_count {
        let monkey = build_monkey(
            i,
            inventories[i].clone(),
            ops[i].clone(),
            divisors[i],
            tfmonkeys[i],
        );
        monkeys.push(monkey);
    }
    let part1 = game_loop(monkeys);
    println!("solution to part 1 is monkey Business: {part1}");
}
