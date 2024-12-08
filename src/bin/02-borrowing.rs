fn main() {
    let s = String::from("Hello, Rust!");
    print_str(&s); // 借用を渡すため、所有権は移動しない
    println!("{}", s); // 所有権は移動していないので、ここで使える
}

fn print_str(text: &String) {
    println!("{}", text); // 借用された文字列を表示

    // text.push_str("!!!"); // 借用された文字列は変更できない
}
