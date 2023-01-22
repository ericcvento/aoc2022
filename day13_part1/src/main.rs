use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::fs;

#[derive(Debug)]
enum ElementKind {
    Int(i32),
    IntList(Vec<i32>),
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    ft
}

fn parse_open_bracket(input: &str) -> IResult<&str, &str> {
    tag("[")(input)
}

fn parse_closed_bracket(input: &str) -> IResult<&str, &str> {
    tag("]")(input)
}

fn recognize_int(input: &str) -> IResult<&str, i32> {
    let (rem, number) = is_a("12345678910")(input)?;
    Ok((rem, number.parse::<i32>().unwrap()))
}

//I think I want this function to return an ElementKind (either a Vec<i32> or i32).
//separated_list returns a Vec<_>,
//recognize_int returns an i32,
//what should parse_int_list return?
fn read_element_list(input: &str) -> IResult<&str, ElementKind> {
    separated_list0(tag(","), alt((recognize_int, parse_int_list)))(input)
}

fn parse_int_list(input: &str) -> IResult<&str, ElementKind> {
    let mut remaining = input;
    let (remaining, _) = parse_open_bracket(remaining)?;
    let (remaining, ints) = read_int_list(remaining)?;
    let (remaining, _) = parse_closed_bracket(remaining)?;
    Ok((remaining, ints))
}

fn main() {
    let input_text = read_data();
    for l in input_text.lines() {
        println!("{l}");
        if l.is_empty() {
            continue;
        }

        println!("{:?}", parse_int_list(l))
    }
}
