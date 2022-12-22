use std::fs;
use substring::Substring;

fn main() {
    part1(); 
}

fn part1() {
    let filetext: String = fs::read_to_string(r"data\day2input.txt").expect("Invalid File.");
    let mut score: i32 = 0;
    for line in filetext.lines() {
        let myplaychar: &str = line.substring(2, 3);
        let yourplaychar: &str = line.substring(0, 1);
        //X for Rock, Y for Paper, and Z for Scissors
        //shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
        let mut myplay: &str = "";
        let mut myplaypoints: i32 = 0;
        if myplaychar == "X" {
            myplay = "Rock";
            myplaypoints = 1;
        } else if myplaychar == "Y" {
            myplay = "Paper";
            myplaypoints = 2;
        } else if myplaychar == "Z" {
            myplay = "Scissors";
            myplaypoints = 3;
        }
        //A for Rock, B for Paper, and C for Scissors
        let mut yourplay: &str = "";
        let mut yourplaypoints: i32 = 0;
        if yourplaychar == "A" {
            yourplay = "Rock";
            yourplaypoints = 1;
        } else if yourplaychar == "B" {
            yourplay = "Paper";
            yourplaypoints = 2;
        } else if yourplaychar == "C" {
            yourplay = "Scissors";
            yourplaypoints = 3;
        }

        let mut outcome: i32 = 0;
        if ((myplay == "Rock") & (yourplay == "Scissors"))
            | ((myplay == "Paper") & (yourplay == "Rock"))
            | ((myplay == "Scissors") & (yourplay == "Paper"))
        {
            outcome = 6;
        } else if myplay == yourplay {
            outcome = 3;
        }
        score += myplaypoints + outcome;
    }
    println!("My Score: {}", score);
}