use std::fmt::Display;

fn longest_with_bound<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: PartialOrd + Display,
{
    println!("Comparing {} and {}", x, y);
    if x > y {
        x
    } else {
        y
    }
}

fn main(){
    let s1 = "long";
    let s2 = "longer";

    let result = longest_with_bound(&s1, &s2);
    println!("The longest string is: {}", result);
}