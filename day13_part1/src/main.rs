use std::fs;

use nom::{
    IResult,
    sequence::delimited,
    character::complete::char,
    bytes::complete::is_not,
    multi::separated_list0,
    bytes::complete::tag
  };
  
  fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
  }

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    ft
}

fn parser(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag("],"),tag(""))(s)
}

fn main() {
    let input_text = read_data();
    for line in input_text.lines() {
        println!("{line}");
        println!("{:?}",parens(&line));
    }
}
