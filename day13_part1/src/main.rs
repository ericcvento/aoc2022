use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::fs;

#[derive(Debug, Clone)]
enum ElementKind {
    Int(i32),
    List(Vec<ElementKind>),
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

fn recognize_int(input: &str) -> IResult<&str, ElementKind> {
    let (rem, number) = is_a("12345678910")(input)?;
    Ok((rem, ElementKind::Int(number.parse::<i32>().unwrap())))
}

fn read_element_list(input: &str) -> IResult<&str, Vec<ElementKind>> {
    separated_list0(tag(","), alt((recognize_int, parse_int_list)))(input)
}

fn parse_int_list(input: &str) -> IResult<&str, ElementKind> {
    let remaining = input;
    let (remaining, _) = parse_open_bracket(remaining)?;
    let (remaining, ints) = read_element_list(remaining)?;
    let (remaining, _) = parse_closed_bracket(remaining)?;
    Ok((remaining, ElementKind::List(ints)))
}

fn compare_left_right(left: &ElementKind, right: &ElementKind) {
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
}

fn main() {
    let input_text = read_data();

    let mut left: ElementKind = ElementKind::Int(0);
    let mut right: ElementKind = ElementKind::Int(0);
    let mut i = 1;
    for l in input_text.lines() {
        if l.is_empty() {
            i = 1;
            println!("");
            continue;
        }
        match i {
            1 => (_, left) = parse_int_list(l).unwrap(),
            2 => {
                (_, right) = parse_int_list(l).unwrap();
                compare_left_right(&left, &right)
            }
            _ => {}
        }
        i += 1;
    }
}
