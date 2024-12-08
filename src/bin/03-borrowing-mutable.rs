fn main() {
    let mut s = String::from("Hello");
    append_world(&mut s); // ミュータブルな借用を渡す
    print!("{}", s); // 所有権は移動していないので、ここで使える
}

fn append_world(text: &mut String) {
    text.push_str(", World!");
}
