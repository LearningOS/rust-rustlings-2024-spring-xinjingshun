// errors6.rs
//
/*
使用像 Box<dyn error::Error> 这样的通用错误类型在库代码中通常不推荐，因为调用者可能需要根据错误的具体内容来做出决策，而不仅仅是打印出来或者进一步传播错误。在这里，我们定义一个自定义错误类型，使得当我们的函数返回错误时，调用者可以决定接下来要做什么。
 */
//
// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

// 这是一个自定义错误类型，用于 `parse_pos_nonzero()` 函数。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError { //定义了在解析和验证输入时可能遇到的错误类型。其中包括字符串解析错误（ParseIntError）和创建 PositiveNonzeroInteger 的自定义错误（CreationError）。
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: add another error conversion function here.
    // 完成 TODO: 添加另一个错误转换函数,将 ParseIntError 转换为我们自定义的错误类型 ParsePosNonzeroError。
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> { //在函数中具体处理上面定义的枚举类型错误
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    // let x: i64 = s.parse().unwrap(); 修改此行代码,map_err() 将 parse() 方法产生的 ParseIntError 转换为我们的自定义错误类型，确保不会在解析失败时panic!
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
