use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::cmp::max;
use std::cmp::min;
use std::fs;

#[derive(Debug, Clone)]
enum ElementKind {
    Int(i32),
    List(Vec<ElementKind>),
}

fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    //let ft: String = fs::read_to_string(r"data\day13test.txt").expect("Invalid File.");
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

fn step_through_elementkind(
    input: ElementKind,
    intermediate: &mut Vec<ElementKind>,
) -> Vec<ElementKind> {
    match input {
        ElementKind::List(items) => {
            for item in items {
                intermediate.push(item.clone());
                step_through_elementkind(item, intermediate);
            }
        }
        ElementKind::Int(_) => {}
    }
    intermediate.to_vec()
}

fn return_next_level(input: &ElementKind) -> Vec<ElementKind> {
    let mut output = Vec::new();
    if let ElementKind::List(items) = input {
        for item in items {
            output.push(item.clone());
        }
    }
    output
}

fn compare_elements_1(left: &ElementKind, right: &ElementKind) {
    //return top level items
    let lefts = return_next_level(left);
    let rights = return_next_level(right);
    compare_elements_n(lefts, rights);
}

fn compare_elements_n(left: Vec<ElementKind>, right: Vec<ElementKind>) {
    println!("{:?}", left);
    println!("{:?}", right);
}

fn main() {
    let input_text = read_data();
    let mut i = 1;

    //main loop
    for l in input_text.lines() {
        if l.is_empty() {
            i = 1;
            println!("{l}");
            continue;
        }
        println!("{i}: {l}");
        let parsed = parse_int_list(l).unwrap().1;
        i += 1;
    }
}
