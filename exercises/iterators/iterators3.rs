// iterators3.rs
//
// 这是一个比大多数其他练习都要大的练习！你可以做到的！如果你选择接受这个任务，这是你的：
// 1. 完成 divide 函数以通过前四个测试。
// 2. 通过完成 result_with_list 和 list_of_results 函数来通过剩余的测试。
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 计算 a 除以 b，如果 a 能被 b 整除。
// 否则，返回一个合适的错误。
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0{
        Err(DivisionError::DivideByZero)
    }else if a%b>0{
        Err(DivisionError::NotDivisible(NotDivisibleError{dividend:a,divisor:b
        }))
    }else{
        Ok(a/b)
    }
}

// 完成函数并返回正确类型的值，以便测试通过。
// 期望输出：Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>,DivisionError>{
    let numbers = vec![27, 297, 38502, 81];
    let mut results = Vec::new();
    for number in numbers{
        match divide(number,27){
            Ok(result)=> results.push(result),
            Err(_e) =>  panic!()
        }
    }
    Ok(results)
}

// 完成函数并返回正确类型的值，以便测试通过。
// 期望输出：[Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32,DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];

    let division_results:Vec<Result<i32,DivisionError>> = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
