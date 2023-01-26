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

fn step_through_elementkind(input: ElementKind, intermediate: &mut Vec<i32>) -> Vec<i32> {
    if let ElementKind::List(items) = input {
        for item in items {
            match item {
                ElementKind::List(recall) => {
                    step_through_elementkind(ElementKind::List(recall), intermediate);
                }
                ElementKind::Int(int) => {
                    intermediate.push(int);
                }
            }
        }
    }
    intermediate.to_vec()
}

fn main() {
    let input_text = read_data();
    let mut i = 1;
    let mut j: i32 = 1;
    let mut score: i32 = 0;
    let mut left_ints = Vec::new();

    for l in input_text.lines() {
        if l.is_empty() {
            i = 1;
            continue;
        }
        println!("{i}: {l}");
        if i == 1 {
            left_ints = step_through_elementkind(parse_int_list(l).unwrap().1, &mut Vec::new());
        }
        if i == 2 {
            let right_ints =
                step_through_elementkind(parse_int_list(l).unwrap().1, &mut Vec::new());

            j += 1;

            if left_ints.len() > right_ints.len() {
                continue;
            }
            for index in 0..left_ints.len() {
                println!("{}-{}", left_ints[index], right_ints[index]);
                if left_ints[index] > right_ints[index] {
                    continue;
                }
            }
            score += j;
        }
        i += 1;
    }
    println!("{score}");
}
