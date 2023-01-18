use std::fs; 
use nom::bytes::complete::tag;
use nom::IResult; 


fn read_data() -> String {
    let ft: String = fs::read_to_string(r"data\day13input.txt").expect("Invalid File.");
    ft
}

fn parser(s: &str) -> IResult<&str, &str> {
    tag("],")(s)
  }

fn return_pairs(output:String) -> Vec<(String,String)> {
    let mut pairs=Vec::new(); 
    let mut p1:String="".to_string(); 
    let mut i=1; 
    for line in output.lines(){
        if i==1 {
            p1=line.to_string(); 
        } else if i==2 {
            pairs.push((p1.clone(),line.to_string())); 
        } else if i==3 {
            i=1; 
            continue;
        }
        i+=1; 
    }
    pairs
}

fn main() {
    let input_text=read_data();
    let pairs=return_pairs(input_text); 
    for p in pairs {
        println!("{:?}",p); 
    };
}
