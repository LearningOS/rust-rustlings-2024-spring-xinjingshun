// tests9.rs
//
// Rust 非常有能力与 C/C++ 和其他静态编译语言共享 FFI 接口，它甚至可以在代码本身内部进行链接！它通过 extern 块来实现，就像下面的代码一样。
//
// `extern` 关键字后面的短字符串表明了外部导入函数将遵循的 ABI（应用二进制接口）。在这个练习中，使用了 "Rust"，同时还有其他变体存在，比如 "C" 用于标准 C ABI，"stdcall" 用于 Windows ABI。
//
// 外部导入的函数在 extern 块中声明，用分号而不是大括号来标记签名的结束。可以对这些函数声明应用一些属性来修改链接行为，例如使用 #[link_name = ".."] 来修改实际的符号名称。
//
// 如果你想要将你的符号导出到链接环境中，`extern` 关键字也可以标记在具有相同 ABI 字符串注释的函数定义之前。Rust 函数的默认 ABI 就是 "Rust"，所以如果你想链接纯 Rust 函数，整个 extern 术语可以省略。
//
// Rust 默认会对符号进行名称修饰（mangling），就像 C++ 那样。为了抑制这种行为并使这些函数可以通过名称进行寻址，可以应用属性 #[no_mangle]。
//
// 在这个练习中，你的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用相同的函数。
//
// 除了添加两行属性代码外，你不应该修改任何现有的代码。

// I AM DONE

extern "Rust" {

    fn my_demo_function(a: u32) -> u32;
    // 使用 link_name 属性指定函数的实际符号名称，确保在链接时能够正确地找到对应的函数。
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

// 定义模块 Foo，其中包含一个名为 my_demo_function 的函数，该函数将参数原样返回。
// 由于没有使用 extern 关键字，这意味着该函数的 ABI 与 Rust 的默认 ABI（也是 "Rust"）相同。
mod foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // 外部导入的函数默认是不安全的，因为它们来自于其他语言，可能存在安全问题。
        // 我们知道这些函数实际上是 Rust 函数的别名，因此我们可以使用 unsafe 块调用它们。
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
