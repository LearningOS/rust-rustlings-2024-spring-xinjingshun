// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

//
// 除了添加或删除引用外，不要更改任何内容。
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data:&String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let mut data = data.to_uppercase();

    println!("{}", data);
}
