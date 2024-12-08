fn main(){
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(num) => println!("The number is {}", num),
        None => println!("There is no number!"),
    }

    if let Some(num) = maybe_number{ // if letを使うと、match式を簡潔に書ける
        println!("if let: The number is {}", num);
    }
}
