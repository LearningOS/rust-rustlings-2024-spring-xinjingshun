// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: String) -> bool {
    let attempt:&str = &attempt as &str;
    attempt == "green" || attempt == "blue" || attempt == "red" //检查attempt是否等于字符串"green"、"blue"或"red"中的任何一个。如果attempt与这些颜色词中的任何一个相匹配，函数将返回true，否则返回false。
}
