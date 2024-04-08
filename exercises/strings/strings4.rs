// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// 这里有一堆值——有些是“String”，有些是“&str”。您的任务是根据您认为每个值是什么，对每个值调用这两个函数之一。也就是说，在每行的括号前添加“string_slice”或“string”。如果你是对的，它会编译！
// No hints this time!

// I AM DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); //to_owned()方法用于创建一个从借用数据（比如&str或&[T]）到拥有数据（比如String或Vec<T>）的副本
    string("nice weather".into());//它允许一种类型的值转换成另一种类型的值。这通常涉及到资源的所有权转移。
    string(format!("Interpolation {}", "Station"));// format!宏用于创建一个格式化的字符串。它返回的数据类型是String
    string_slice(&String::from("abc")[0..1]); // 对String类型使用范围表达式时，你会得到一个&str类型的借用
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
