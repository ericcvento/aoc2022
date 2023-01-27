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

fn compare_left_right(left: &ElementKind, right: &ElementKind) {
    if let ElementKind::List(left_items)=left {
        let left_length=left_items.len(); 
        println!("{}",left_length);
        for i in left_items {
            println!("{:?}",i); 
        } 
    }
    if let ElementKind::List(right_items)=right {
        let right_length=right_items.len(); 
        println!("{}",right_length); 
        for i in right_items {
            println!("{:?}",i); 
        } 
    }
}

10,[10,3,[0,5,9,3],[0,3,3,6,6]]