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
