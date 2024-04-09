// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
// 您可以使用 'use' 关键字将模块路径从任何地方，尤其是 Rust 标准库中的模块路径引入您的范围。从 std：：time 模块引入 SystemTime 和 UNIX_EPOCH。如果你能用一行做的话，奖励风格点！
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

// TODO: Complete this use statement
use std::time::{SystemTime,UNIX_EPOCH};  //EP零CH，不是大写字母O

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
