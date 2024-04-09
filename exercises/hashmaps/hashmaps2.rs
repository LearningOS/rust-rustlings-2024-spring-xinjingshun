// hashmaps2.rs
//
// 我们正在收集不同的水果来烘烤美味的水果蛋糕。为此，
// 我们有一个篮子，我们将以哈希图的形式表示。键表示我们收集的每个水果的名称，值表示如何我们收集了许多特殊的水果。三种水果——
// 苹果（4）、芒果（2）和荔枝（5）已经在篮子哈希图中。你必须将水果添加到篮子中，以便每种水果至少有一种，并且
// 总共超过 11 个 - 我们有很多嘴要养活。你不被允许插入更多这些水果！
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: 如果篮子里还没有新的水果，请插入它们。请注意，您不允许放置任何已经存在的水果！
        // 检查水果是否已经在篮子中
        if !basket.contains_key(&fruit) {
            // 如果水果尚未存在，则将其插入篮子中,强买强卖。
            basket.insert(fruit, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }

    #[test]
    fn all_fruit_types_in_basket() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        for amount in basket.values() {
            assert_ne!(amount, &0);
        }
    }
}
