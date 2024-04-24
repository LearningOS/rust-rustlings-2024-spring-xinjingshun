// using_as.rs
//

// 在 Rust 中，类型转换是通过使用 as 运算符完成的。请注意，as 运算符不仅用于类型转换，它还用于给导入项重命名。。
//
// 目标是确保除法不会导致编译失败，并返回正确的类型。
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

// I AM
// DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / (values.len() as f64)
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
