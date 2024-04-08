// structs1.rs
//
// 解决所有待办事项以使测试通过！
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

//

struct ColorClassicStruct<'a> {
    // TODO: Something goes here
    color:&'a str,}
// 定义了一个名为 ColorClassicStruct 的 Rust 结构体，它使用生命周期参数 'a。生命周期参数 'a 指定了结构体中 color 字段的引用可以存活的时间。在这个结构体中，color 是一个对字符串切片（&str）的引用，它的生命周期与 'a 相关联

struct ColorTupleStruct(u8,u8,u8);

#[derive(Debug)]
//单元结构体(Unit-like Struct)
struct UnitLikeStruct;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // 实例化一个经典的 c 结构体！
        struct Green{
            red:u8,
            green:u8,
            blue:u8,
        };
        let green = Green{
            red : 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = (0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // 实例化一个类似单元的结构体！
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
