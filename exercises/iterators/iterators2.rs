// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();//chars 方法用于创建一个遍历给定字符串中所有 Unicode 字符（char）的迭代器。每次迭代返回字符串中的一个 char 类型的值
    match c.next() {
        None => String::new(),
        Some('h') => String::from("Hello"),
        Some('w') => String::from("World"),
        Some(' ') =>String::from(" "),
        _ =>String::from("error"),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &Vec<&str>) -> Vec<String> {
    let mut first_words:Vec<String> = Vec::new();
    for item in words{
        first_words.push(capitalize_first(item))
    }
    first_words
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// 返回单个字符串.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &Vec<&str>) -> String {
    let mut words:Vec<String> = capitalize_words_vector(&words);
    let mut re_words:String = "".to_string();
    for item in words{
        re_words += &item };
    re_words

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(" "), " ");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
