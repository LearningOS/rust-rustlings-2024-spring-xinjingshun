// from_into.rs
//

// From trait 用于值到值的转换。如果针对某个类型正确实现了 From，那么 Into trait 应该可以相反地工作。您可以在 https://doc.rust-lang.org/std/convert/trait.From.html 上了解更多信息。
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


// 我们实现了 Default trait 以便在提供的字符串无法转换为 Person 对象时使用它作为默认值。
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 您的任务是完成此实现，以便使 let p = Person::from("Mark,20") 这行代码能够编译。请注意，您需要使用类似 "4".parse::<usize>() 的方法将年龄部分解析为 usize。需要适当处理这个操作的结果。
//
// 步骤：
// 1. 如果提供的字符串长度为 0，则返回 Person 的默认值。
// 2. 在字符串中出现的逗号上分割给定的字符串。
// 3. 从分割操作中提取第一个元素，并将其用作名称。
// 4. 如果名称为空，则返回 Person 的默认值。
// 5. 从分割操作中提取另一个元素，并将其解析为 usize 类型作为年龄。
// 如果在解析年龄时出现问题，则返回 Person 的默认值。否则，返回一个使用结果实例化的 Person 对象。

// I AM DONE

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }

        if let Some((person_name, person_age)) = s.split_once(',') {
            if person_name.is_empty() {
                return Person::default();
            }

            if let Ok(age) = person_age.trim().parse::<usize>() {
                return Person {
                    name: person_name.to_string(),
                    age,
                };
            }
        }

        Person::default()
    }
}


fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
