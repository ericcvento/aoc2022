use std::fs;

fn main() {
    day1();
}

fn day1() {
    let day1filetext: String = fs::read_to_string(r"data\day1.txt").expect("Invalid File.");

    let mut cal_sum: i32 = 0;
    let mut cal_the_big_one = 0;
    for cal in day1filetext.lines() {
        if cal.is_empty() {
            if cal_sum > cal_the_big_one {
                cal_the_big_one = cal_sum;
            }
            cal_sum = 0;
        } else {
            let cal_int: i32 = cal.parse().unwrap();
            cal_sum += cal_int;
        }
    }
    println!("this is the big one: {}", cal_the_big_one);
}
