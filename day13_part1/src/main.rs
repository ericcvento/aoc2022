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

fn unwind_element_kind(
    input: ElementKind,
    group: i32,
    ints: &mut Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    match input {
        ElementKind::List(items) => {
            for item in items {
                unwind_element_kind(item, group + 1, ints);
            }
        }
        ElementKind::Int(int) => {
            println!("{group}---{int}");
            ints.push((group, int));
        }
    }
    ints.to_vec()
}

fn main() {
    let input_text = read_data();
    for l in input_text.lines() {
        println!("{l}");
        if l.is_empty() {
            continue;
        }
        let (_, parsed) = parse_int_list(l).unwrap();
        println!("{:?}", parsed);

        let lk: &mut Vec<(i32, i32)> = &mut Vec::new();
        println!("{:?}", unwind_element_kind(parsed, 0, lk));
    }
}
