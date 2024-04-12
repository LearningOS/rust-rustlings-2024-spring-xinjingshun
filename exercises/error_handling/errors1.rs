// errors1.rs
//
// 这个函数拒绝生成要打印在名牌上的文本，如果你传递给它一个空字符串。如果它解释一下问题所在，而不是仅仅有时返回 None，会更好。幸运的是，Rust 有一个类似于 Result 的结构，可以用来表达错误条件。让我们来使用它吧！
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into()) //into() 方法是 Rust 中的一个 trait 方法，它用于值的转换。具体来说，into() 方法用于将一个类型转换成另一个类型。
    } else {
        Ok(format!("Hi! My name is {}", name)) //  format! 宏用于创建一个格式化的字符串，直接返回了一个 String 类型的值,注意这里不能用println！
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
