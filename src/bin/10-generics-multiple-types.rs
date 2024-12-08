struct Point<T, U>{
    x: T,
    y: U,
}

fn main(){
    let int_and_float = Point{x: 5, y: 4.0};
    let char_and_string = Point{x: 'u', y: "nicorns"};

    println!("int_and_float: x = {}, y = {}", int_and_float.x, int_and_float.y);
    println!("char_and_string: x = {}, y = {}", char_and_string.x, char_and_string.y);
}
