struct Point<T, U>{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>{
    fn x(&self) -> &T{
        &self.x
    }
    fn y(&self) -> &U{
        &self.y
    }
}

fn main(){
    let p1 = Point{x: 3, y: 4.5};
    println!("p1.x = {}", p1.x());
    println!("p1.y = {}", p1.y());

    let p2 = Point{x: 'v', y: "alues"};
    println!("p2.x = {}", p2.x());
    println!("p2.y = {}", p2.y());
}
