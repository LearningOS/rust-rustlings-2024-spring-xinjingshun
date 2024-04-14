// traits2.rs
//
// 你的任务是为字符串向量实现 AppendBar 特性。为了实现这个特性，先考虑一下向字符串向量中“附加 'Bar'”意味着什么。
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
