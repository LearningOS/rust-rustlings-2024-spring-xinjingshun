// tests7.rs
//
//在构建包时，一些依赖项既不能在 `Cargo.toml` 中导入，也不能直接链接；一些预处理步骤从代码生成到设置特定于包的配置各不相同。
//
// Cargo 并不旨在取代其他构建工具，但它确实通过称为 `build.rs` 的自定义构建脚本与它们集成。这个文件通常放置在项目的根目录中，而在本例中，它位于相同目录中的练习中。
//
// 它可用于：
//
// - 构建一个捆绑的 C 库。
// - 在宿主系统上查找 C 库。
// - 从规范生成 Rust 模块。
// - 执行包所需的任何特定于平台的配置。
//
// 在设置配置时，我们可以在构建脚本中使用 `println!` 来指示 Cargo 遵循一些指令。通用格式为：
//
//     println!("cargo:{}", 你的字符串命令);
//
// 有关构建脚本的更多信息，请参阅官方 Cargo 书籍：
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// 在这个练习中，我们查找一个环境变量，并期望它落在一个范围内。你可以查看测试用例以了解详细信息。
//
// 你不应该修改这个文件。要通过这个练习，请修改同目录下的 `build.rs` 文件。
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
