use std::fmt::Display;

fn print_largest<T>(list: &[T])
where
    T: PartialOrd + Display,
{
    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    println!("The largest item is: {}", max);
}

fn main() {
    let numbers = vec![3, 5, 1, 2, 4];
    print_largest(&numbers);

    let chars = vec!['v', 'a', 'l', 'u', 'e', 's'];
    print_largest(&chars);
}
