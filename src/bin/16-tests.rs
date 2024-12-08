fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(3, 5);
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add_positive_numbers(){
        assert_eq!(add(3, 1), 4);
    }

    #[test]
    fn test_add_negative_numbers(){
        assert_eq!(add(-5, -6), -11);
    }
}