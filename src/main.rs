use std::collections::HashMap;
fn main() {
   let mut hash=HashMap::new();
    hash.insert(String::from("pawan"),10);
    hash.insert(String::from("bisht"),20);

    for(key,value) in &hash{
        println!("{} - {}",key,value);
    }

    println!("{:?}",&hash);

    hash.insert(String::from("bisht"),50);
    println!("after updating the value of bisht");
    for(key,value) in &hash{
        println!("{} - {}",key,value);
    }
}
