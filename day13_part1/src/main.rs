use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::fs;

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

fn recognize_int(input: &str) -> IResult<&str, &str> {
    is_a("12345678910")(input)
}

fn read_int_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(","), recognize_int)(input)
}

fn parse_int_list(input: &str) -> IResult<&str, Vec<&str>> {
    let (remaining, _) = parse_open_bracket(input)?;
    let (remaining, ints) = read_int_list(remaining)?;
    let (remaining, _) = parse_closed_bracket(remaining)?;
    Ok((remaining, ints))
}

fn main() {
    let input_text = read_data();
    for l in input_text.lines() {
        println!("{l}");
        println!("{:?}", parse_int_list(l));
    }
}
