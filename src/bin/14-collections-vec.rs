fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(3);
    numbers.push(1);
    numbers.push(4);

    println!("numbers: {:?}", numbers); // {:?} はデバッグフォーマット

    if let Some(value) = numbers.pop() {
        println!("Popped: {}", value);
    } else {
        println!("No value to pop");
    }

    // let first = numbers[0]; // もし要素がない場合はパニックする
    // println!("First element: {}", first);

    match numbers.get(2) {
        Some(value) => println!("Value at index 2: {}", value),
        None => println!("No value at index 2"),
    }

    for num in &numbers {
        println!("number: {}", num);
    }
}
