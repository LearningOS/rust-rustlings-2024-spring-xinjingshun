// errors5.rs
//
/*
这个练习用到了一些我们在课程后面才会讲到的概念，比如 Box 和 From 特征。现在不需要详细理解它们，但如果你感兴趣，可以提前阅读。目前，你可以将 Box<dyn ???> 类型视为“我想要任何实现了 ??? 的东西”的类型，这在 Rust 通常对运行时安全的标准下，应该让你感觉到相当宽松！

简而言之，使用 box 的这种特定情况是当你想拥有一个值，并且只关心它是实现了特定特征的类型时。为此，Box 被声明为 Box<dyn Trait> 类型，其中 Trait 是编译器在该上下文中寻找的任何值上实现的特征。在这个练习中，该上下文是可以在 Result 中返回的潜在错误。

我们能用什么来描述这两种错误？换句话说，有没有一个特征是这两种错误都实现了的？
 */
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
/*
main 函数中可能出现两种错误类型——ParseIntError（由字符串解析函数产生）和 CreationError（由 PositiveNonzeroInteger::new 产生），我们需要返回一个能够包含任何实现了 error::Error 的类型的错误。这可以通过使用 Box<dyn Error> 来实现，它是一个能够包含任何实现了 Error trait 的类型的动态类型箱（Box）。这样，无论是哪种错误类型，都可以被装箱并作为错误返回。
 */
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
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

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
