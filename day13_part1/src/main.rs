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

fn step_through_elementkind(input: ElementKind, output: &mut Vec<i32>) -> Vec<i32> {
    if let ElementKind::List(items) = input {
        for item in items {
            match item {
                ElementKind::List(recall) => {
                    step_through_elementkind(ElementKind::List(recall), output);
                }
                ElementKind::Int(int) => {
                    output.push(int);
                }
            }
        }
    }
    output.to_vec()
}

fn main() {
    let input_text = read_data();
    let mut i = 1;
    for l in input_text.lines() {
        println!("{l}");
        if l.is_empty() {
            i = 1;
            continue;
        }

        let mut left_e: Vec<i32> = Vec::new();
        let left_list = step_through_elementkind(parse_int_list(l).unwrap().1, &mut left_e);
        println!("{:?}", left_list);
        i += 1;
    }
}
