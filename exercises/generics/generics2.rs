// generics2.rs
//
// 这个强大的包装器提供了存储正整数值的能力。使用泛型重写它，以便它支持包装任何类型。
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
