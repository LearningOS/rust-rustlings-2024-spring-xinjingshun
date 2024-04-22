// cow1.rs
//
// 本练习探讨了 Cow，或称为 Clone-On-Write 类型。Cow 是一个惰性克隆的智能指针。它可以封装并提供对借用数据的不可变访问，并在需要进行突变或拥有权时惰性克隆数据。该类型旨在通过 Borrow trait 与通用借用数据一起使用。

// 本练习旨在向您展示将数据传递给 Cow 时可以期望什么。通过在 TODO 标记处检查 Cow::Owned() 和 Cow::Borrowed()，修复单元测试。
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

// I AM DONE

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() { // .len()将得到input的数字的长度，比如128长度是3
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v; //当你使用 `to_mut()` 方法时，它将检查当前 `Cow` 实例是否已经是所有者实例。如果不是，则会克隆该实例到一个新的所有者实例，然后返回 mutable 的引用。
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // 我们也可以不使用 `&` 将 `slice` 传递给 `Cow`，这样 `Cow` 就直接拥有它。 在这种情况下，没有 mutation 发生，也没有克隆，但是结果仍然是所有者的，因为从来没有被借用或修改过。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {

        //当发生mutation时，情况当然也是如此。在这种情况下，对 to_mut() 的调用将返回与之前相同的数据的引用。
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
