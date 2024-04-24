// try_from_into.rs
//
// TryFrom 是一种简单且安全的类型转换，可能在某些情况下以受控的方式失败。基本上，这与 From 是相同的。
// 主要区别在于，这应该返回一个 Result 类型，而不是目标类型本身。您可以在以下链接阅读更多关于它的内容：
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 我们将使用这个错误类型来处理这些 `TryFrom` 转换。
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// I AM DONE

// 你的任务是完成这个实现并返回内部类型 Color 的 Ok 结果。你需要为三个整数的元组、三个整数的数组和整数的切片创建一个实现。
// 注意，元组和数组的实现将在编译时进行检查，但切片的实现需要检查切片的长度！另请注意，正确的 RGB 颜色值必须是介于 0 到 255 范围内的整数。

// 元组实现
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        // 检查 RGB 色彩值是否在 0 到 255 范围内的整数
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color { red: red as u8, green: green as u8, blue: blue as u8 })
    }
}

// 数组实现
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let [red, green, blue] = arr;
        // 检查 RGB 色彩值是否在 0 到 255 范围内的整数
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color { red: red as u8, green: green as u8, blue: blue as u8 })
    }
}

// 切片实现
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // 检查切片长度是否为 3
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        let red = slice[0];
        let green = slice[1];
        let blue = slice[2];
        // 检查 RGB 色彩值是否在 0 到 255 范围内的整数
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color { red: red as u8, green: green as u8, blue: blue as u8 })
    }
}

fn main() {
    // 使用 `try_from` 函数
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // 由于对 Color 实现了 TryFrom，我们应该能够使用 TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // 对切片，我们应该使用 `try_from` 函数
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // 或者在圆括号内取切片，然后使用 TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
