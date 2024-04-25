// tests6.rs
//
//在这个示例中，我们对 Rust 标准库中的不安全函数进行了初步探索。请修复所有的问号和待办事项，以便使测试成功通过。
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: ptr 包含一个 Foo 的拥有权box。我们只需从该指针重建box。
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr)};
    ret.b = Some("hello".to_owned()); // 设置 b 字段的值
    // todo!("The rest of the code goes here")
    println!("Foo is {:?}",ret.a);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
