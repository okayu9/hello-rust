fn main(){
    let s1 = String::from("Hello");
    let s2 = String::from("Rustacean"); // RustaceanはRustのユーザーを指す

    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);
}

// 返す参照は引数のいずれかと同じライフタイムである必要がある
// <'a> はライフタイムパラメータ
// この関数は、引数xとy、返り値のライフタイムが全て同じであることを示している
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
