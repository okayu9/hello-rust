use std::collections::HashMap;

fn main(){
    let mut ages  = HashMap::new();

    ages.insert(String::from("Cocoa"), 15);
    ages.insert(String::from("Chino"), 13);
    ages.insert(String::from("Rize"), 16);

    println!("Ages: {:?}", ages);

    let name = String::from("Cocoa");
    match ages.get(&name) {
        Some(&score) => println!("Score for {}: {}", name, score),
        None => println!("No score found for {}", name),
    }

    for (name, score) in &ages {
        println!("{}: {}", name, score);
    }

    ages.insert(String::from("Cocoa"), 16);
    println!("Updated ages: {:?}", ages);

    ages.entry(String::from("Chiya")).or_insert(15);
    println!("After inserting Chiya: {:?}", ages);

    let chino_score = ages.entry(String::from("Chino")).or_insert(0);
    *chino_score += 1;
    println!("After incrementing Chino's score: {:?}", ages);

    ages.remove(&String::from("Rize"));
    println!("After removing Rize: {:?}", ages);
}