// clippy1.rs
//

// Clippy 工具是一组 lint，用于分析你的代码，以便发现常见错误并改进你的 Rust 代码。
//
// 在这些练习中，当出现 Clippy 警告时，代码将无法编译，请检查 Clippy 的建议来解决练习。
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::f32;
use core::f32::consts::PI;


fn main() {
    // const PI:f32 = 3.14;
    let radius = 5.00f32;

    let area = PI * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
