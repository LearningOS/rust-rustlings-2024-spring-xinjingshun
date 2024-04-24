// as_ref_mut.rs
//
// AsRef 和 AsMut 允许进行廉价的引用到引用的转换。分别在以下链接阅读更多关于它们的信息:
// https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// I AM DONE
use std::convert::{AsRef, AsMut};

// 获取给定参数的字节数（而不是字符数）。
// TODO: 适当添加 AsRef trait 作为 trait bound。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 获取给定参数的字符数（而不是字节数）。
// TODO: 适当添加 AsRef trait 作为 trait bound。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 as_mut() 计算一个数字的平方。
// TODO: 添加适当的 trait bound。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: 实现函数体。
    *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}