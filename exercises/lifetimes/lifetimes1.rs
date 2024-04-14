// lifetimes1.rs
//
// Rust编译器需要知道如何检查提供的引用是否有效，以便在引用可能在使用之前超出范围时通知程序员。请记住，引用是借用，它们不拥有自己的数据。如果它们的所有者超出范围会发生什么？
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn longest<'bbc>(x: &'bbc str, y: &'bbc str) -> &'bbc str { //生命周期的标注用不一定用字符a，不同的字符代表不一样的周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
