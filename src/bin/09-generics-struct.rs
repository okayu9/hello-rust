struct Point<T>{
    x: T,
    y: T,
}

fn main(){
    let integer_point = Point{x: 5, y: 10};
    let float_point = Point{x: 1.0, y: 4.0};

    println!("integer_point: x = {}, y = {}", integer_point.x, integer_point.y);
    println!("float_point: x = {}, y = {}", float_point.x, float_point.y);
}
