use std::fs;
use substring::Substring;

fn main() {
    part1();
    part2();
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

fn part2() {
    let filetext: String = fs::read_to_string(r"data\day2input.txt").expect("Invalid File.");
    let mut score: i32 = 0;
    for line in filetext.lines() {
        let myplaychar: &str = line.substring(2, 3);
        let yourplaychar: &str = line.substring(0, 1);
        //A for Rock, B for Paper, and C for Scissors
        let mut yourplay: &str = "";
        if yourplaychar == "A" {
            yourplay = "Rock";
        } else if yourplaychar == "B" {
            yourplay = "Paper";
        } else if yourplaychar == "C" {
            yourplay = "Scissors";
        }
        //X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
        let mut outcome = 0;
        if myplaychar == "X" {
            outcome = 0;
        } else if myplaychar == "Y" {
            outcome = 3;
        } else if myplaychar == "Z" {
            outcome = 6;
        }
        let mut myplay: &str = "";
        let mut myplaypoints = 0;
        //1 for Rock, 2 for Paper, and 3 for Scissors
        if yourplay == "Rock" {
            if outcome == 0 {
                myplay = "Scissors";
                myplaypoints = 3;
            } else if outcome == 3 {
                myplay = "Rock";
                myplaypoints = 1;
            } else if outcome == 6 {
                myplay = "Paper";
                myplaypoints = 2;
            }
        }
        if yourplay == "Paper" {
            if outcome == 0 {
                myplay = "Rock";
                myplaypoints = 1;
            } else if outcome == 3 {
                myplay = "Paper";
                myplaypoints = 2;
            } else if outcome == 6 {
                myplay = "Scissors";
                myplaypoints = 3;
            }
        }
        if yourplay == "Scissors" {
            if outcome == 0 {
                myplay = "Paper";
                myplaypoints = 2;
            } else if outcome == 3 {
                myplay = "Scissors";
                myplaypoints = 3;
            } else if outcome == 6 {
                myplay = "Rock";
                myplaypoints = 1;
            }
        }
        score += outcome + myplaypoints;
    }
    println!("My Score: {}", score);
}
