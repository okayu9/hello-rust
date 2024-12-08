fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    let sum = add(3, 5);
    println!("sum: {}", sum);

    println!("Is 3 even? {}", is_even(3));
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

    #[test]
    fn test_is_even_true() {
        assert!(is_even(2), "2 should be even");
    }

    #[test]
    fn test_is_even_false() {
        assert!(!is_even(5), "5 should not be even");
    }
}