fn main() {
    let s1 = String::from("Hello, Rust!");
    let s2 = s1; // s1の所有権をs2に移動

    // s1は所有権を失ったので、以下の行はコンパイルエラー
    // println!("{}", s1);

    println!("{}", s2); // s2は有効
}
