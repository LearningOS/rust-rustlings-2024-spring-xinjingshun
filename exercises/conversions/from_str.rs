// from_str.rs
//
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr` 并返回错误，而不是回退到默认值。
// 此外，实现 FromStr 后，您可以在字符串上使用 `parse` 方法生成实现类型的对象。您可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 上了解更多信息。

//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将在 FromStr 实现中使用这种错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError { // 自定义错误类型 `ParsePersonError`
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// I AM DONE

// 步骤:
// 1. 如果提供的字符串长度为 0，则应返回错误
// 2. 在其中出现的逗号上分割给定的字符串
// 3. 分割操作应返回2个元素，否则返回错误
// 4. 从分割操作中提取第一个元素，并将其用作名称
// 5. 从分割操作中提取另一个元素，并将其解析为 `usize` 类型作为年龄，例如使用 `"4".parse::<usize>()`
// 6. 如果在提取名称和年龄时出现问题，则应返回错误
// 如果一切顺利，则返回一个包含 Person 对象的 Result

// 顺便说一句: `Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着，如果要返回一个字符串错误消息，只需使用 `return Err("my error message".into())`。


impl FromStr for Person {
    // 自定义错误类型 `ParsePersonError`
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 如果字符串为空，返回Empty
        if s.is_empty(){
            return Err(ParsePersonError::Empty);
        }

        // 使用逗号分割字符串，并将结果收集到 `fields` 向量中
        let fields: Vec<&str> = s.split(',').collect();
        // 如果分割后的字段数量不为 2，则返回 `BadLen` 错误
        if fields.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 提取姓名和年龄
        let name = fields[0];
        let age = fields[1];

        // 如果姓名为空，则返回 `NoName` 错误
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 将年龄部分解析为 `usize` 类型
        let age = match age.trim().parse::<usize>() {
            // 解析成功，返回年龄
            Ok(age) => age,
            // 解析失败，返回 `ParseInt` 错误
            Err(e) => return Err(ParsePersonError::ParseInt(e)),
        };

        // 构建 `Person` 结构体并返回
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
