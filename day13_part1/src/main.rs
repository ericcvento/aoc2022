use std::fs;
use nom::IResult; 
use nom::character::is_alphabetic;
use nom::error::Error;
use nom::bytes::complete::{tag,take_while};  
use nom::character::complete::{one_of}; 
use nom::multi::separated_list0; 
use nom::sequence::delimited; 

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    ft
}

fn remove_first_bracket (input:&str) -> IResult<&str,&str> {
    tag("[")(input)
}

fn main() {
    let input_text = read_data();
    for l in input_text.lines() {
        println!("{l}");
        println!("{:?}",remove_first_bracket(&l));
    }
}
