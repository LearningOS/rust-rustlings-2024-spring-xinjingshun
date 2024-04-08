// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

//

//给出了偶数的 Vec。您的任务是完成循环，以便将 Vec 中的每个数字乘以 2。
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    //iter_mut() 是一个方法，它允许你以可变引用（&mut T）的形式迭代集合中的每个元素。这意味着你可以在迭代过程中修改这些元素。
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element*2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        //// 创建一个只包含前5个偶数的动态数组v
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // 调用vec_loop函数，并将v的克隆作为参数传递
        let ans = vec_loop(v.clone());
        // 断言vec_loop函数的返回值与v中每个元素乘以2后的结果相等
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
