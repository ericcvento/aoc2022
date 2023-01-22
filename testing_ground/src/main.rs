use nom::bytes::complete::is_a;
use nom::IResult;

#[derive(Debug)]
enum ElementKind {
    Int(i32),
    IntList(Vec<i32>),
}

fn recognize_int(input: &str) -> IResult<&str, ElementKind> {
    let (rem, number) = is_a("12345678910")(input)?;
    Ok((rem, ElementKind::Int(number.parse::<i32>().unwrap())))
}

fn main() {
    println!("{:?}", recognize_int("1,2,3"));
    println!("{:?}", recognize_int("12,3"));
}
