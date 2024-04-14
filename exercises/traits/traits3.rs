// traits3.rs
//
// 你的任务是为两个结构体实现 Licensed 特性，并让它们返回相同的信息，但无需编写两次相同的函数。
//
// 考虑一下你可以在 Licensed 特性中添加什么。
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

pub trait Licensed {
    fn licensing_info(&self) -> String{
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}



struct OtherSoftware {
    version_number: String,
}

// impl Licensed for (SomeSoftware,OtherSoftware){
//     fn licensing_info(&self) -> String{
//         self
//     }
// }

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
