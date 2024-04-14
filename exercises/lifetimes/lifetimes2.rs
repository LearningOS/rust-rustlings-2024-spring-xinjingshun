// lifetimes2.rs
//
// 如果编译器只是验证传递给已标注参数和返回类型的引用，我们需要改变什么呢？
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn longest<'a>(x: &'a str, y: &'a str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
