// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = match total_cost(pretend_user_input){
        Ok(some) => some,
        Err(e) => {
            eprintln!("Error parsing input: {}", e); //比println! 还会将错误传输到错误流，可以写入日志
        -1} //提供个默认值
    };



    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; //使用 ? 运算符来传递错误，使得函数在遇到无法解析的输入时自动返回错误

    Ok(qty * cost_per_item + processing_fee)
}
