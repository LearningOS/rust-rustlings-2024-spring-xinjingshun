// errors2.rs
//
/*设想我们正在编写一个游戏，玩家可以使用代币购买物品。所有物品的价格都是5个代币，每次购买物品时还需要支付1个代币的手续费。游戏玩家会输入他们想要购买的物品数量，而total_cost函数将计算代币的总成本。但是，由于玩家输入的数量是以字符串形式得到的——他们可能输入的不仅仅是数字！

目前，这个函数根本没有处理错误情况（成功情况也没有正确处理）。我们想要做的是：如果我们在一个不是数字的字符串上调用parse函数，该函数将返回一个ParseIntError，在这种情况下，我们希望立即从我们的函数中返回那个错误，并且不进行乘法和加法运算。

实现这一功能有至少两种正确的方法——但其中一种要简短得多！
*/
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>(); //使用 .parse::<i32>() 方法尝试将字符串解析为整数时，如果该字符串不能正确解析成一个整数（比如因为其中包含非数字字符），这个操作并不会导致程序崩溃。相反，这会安全地返回一个包含错误的 Result 类型。
    match qty{
        Ok(input_num)=>Ok(input_num * cost_per_item + processing_fee), //留意，需要返回Result类型。
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
