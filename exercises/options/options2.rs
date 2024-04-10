// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // word = optional_target { 定义变量错误。
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 将这个语句改为while let语句 - 记住，vector.pop 也会添加另一层 Option<T>。你可以将 Option<T> 嵌套在 while let 和 if let 中。
        while let Some(integer) = optional_integers.pop().unwrap(){ // while let匹配成功时执行循环体，直到模式匹配失败。
            println!("{}",integer);
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
