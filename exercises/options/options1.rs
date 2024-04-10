// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// 此函数返回冰箱中还剩下多少冰淇淋。如果是晚上 10 点之前，还剩下 5 件。晚上 10 点，有人把它们都吃掉了，所以不会再剩下了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我们在这里使用 24 小时系统，因此 10PM 的值为 22,12AM 的值为 0, 选项输出应正常处理time_of_day > 23 的情况
    // TODO: Complete the function body - remember to return an Option!
    match time_of_day{
       0..=21_u16 => Some(5), //0..21表示的范围是从0到20，不包括21。
        22 | 23_u16 => Some(0),
        _ => None,
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        // maybe_icecream函数返回的是一个Option<u16>类型的值，而不是直接的u16类型。因此，我们不能直接将icecreams与整数5进行比较
        let icecreams = maybe_icecream(12).unwrap();
        // 也可以用模式匹配取值。
        // let icecreams = maybe_icecream(12);
        // match icecreams {
        //     Some(value) => {
        //         // 当Option是Some时，将其中的值绑定到value变量
        //         assert_eq!(value, 5);
        //     }
        //     None => {
        //         // 当Option是None时，执行相应的逻辑
        //         panic!("Expected Some(5), but got None.");
        //     }
        // }
        assert_eq!(icecreams, 5);
    }
}
