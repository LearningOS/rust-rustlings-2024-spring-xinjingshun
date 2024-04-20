// box1.rs
//
// 在编译时，Rust 需要知道类型占用多少空间。对于递归类型，这会产生问题，因为一个值可以作为其一部分拥有另一个相同类型的值。为了解决这个问题，我们可以使用 Box - 一种智能指针，用于在堆上存储数据，它还允许我们包装递归类型。
//
// 在这个练习中，我们要实现的递归类型是 cons list - 一种在函数式编程语言中经常出现的数据结构。cons 列表中的每个项目都包含两个元素：当前项目的值和下一个项目。最后一项是称为 Nil 的值。
//
// 步骤1：在枚举定义中使用 Box 来使代码编译通过
// 步骤2：通过替换 todo!() 来创建空和非空 cons 列表。
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[derive(PartialEq, Debug)]
pub enum List { //Cons 和 Nil 并不是关键字，它们是自定义的枚举变体的名称
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(5,Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
