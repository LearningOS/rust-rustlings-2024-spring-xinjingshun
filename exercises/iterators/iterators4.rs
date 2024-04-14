// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub fn factorial(num: u64) -> u64 {

    // 完成这个函数，返回数字 num 的阶乘
    // 不要使用：
    // - return
    // 尽量不要使用：
    // - 命令式循环（for，while）
    // - 额外的变量
    // 为了更有挑战性，不要使用：
    // - 递归
    // Execute `rustlings hint iterators4` for hints.
    // 使用 `1..=num` 创建一个包含从 1 到 num 的闭区间的迭代器, 使用迭代器的 fold 方法来累加乘积
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
