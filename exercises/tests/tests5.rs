// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// 当 unsafe 标记在项声明上时，例如一个函数、一个特质等，它同时声明了一个契约。然而，
// 契约的内容不能仅通过单一关键词表达。
// 因此，你有责任在项的文档注释中的 # Safety 部分手动声明它。
//
// 当 unsafe 标记在一个由花括号封闭的代码块上时，
// 它声明了对某些契约的遵守，例如某些指针参数的有效性、某些内存地址的所有权。然而，像
// 上文提到的，你仍然需要在代码块的注释中声明如何遵守契约。
//
// 注意：所有注释都是为了提高代码的可读性和可维护性，而 Rust 编译器将其对代码的安全性的信赖
// 交给了你自己！如果你不能证明你自己代码的内存安全和合理性，退一步，改用安全代码！
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        //将地址转换成可变指针
        let ptr = address as *mut u32;
        *ptr =  0xAABBCCDD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
