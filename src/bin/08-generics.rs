fn main(){
    let numbers = vec![82, 117, 115, 116];
    let max_number = largest(&numbers);
    println!("The largest number is {}", max_number);

    let chars = vec!['g', 'e', 'n', 'e', 'r', 'i', 'c', 's'];
    let max_char = largest(&chars);
    println!("The largest char is {}", max_char);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}