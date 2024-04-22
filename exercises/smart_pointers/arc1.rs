// arc1.rs
//
// 在这个练习中，我们有一个包含从 0 到 99 的值的名为 "numbers" 的 Vec<u32> — [0, 1, 2, ..., 98, 99]。我们想要在 8 个不同的线程中同时使用这组数字。每个线程将对具有偏移量的每八个值进行求和。
//
// 第一个线程（偏移量为 0）将求和 0、8、16、...
// 第二个线程（偏移量为 1）将求和 1、9、17、...
// 第三个线程（偏移量为 2）将求和 2、10、18、...
// ...
// 第八个线程（偏移量为 7）将求和 7、15、23、...
//
// 因为我们使用了线程，所以我们的值需要是线程安全的。因此，我们使用了 Arc。我们需要在两个 TODO 中做出更改。
//
// 在第一个 TODO 的注释中填入 shared_numbers 的值，使得该代码能够编译通过，并在第二个 TODO 注释中为 child_numbers 创建初始绑定。尽量不要创建 numbers 的任何副本！
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);// 创建一个 Arc，使其拥有 numbers 的所有权，并将其存储在堆上。Arc 允许多个线程共享所有权，因为 `numbers` 会被多个线程共享，所以我们使用了 Arc。
    let mut joinhandles = Vec::new();   // 创建一个用于存储线程句柄的 Vec

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);// 使用 Arc::clone 创建了一个新的 Arc 指针，将其与 shared_numbers 指向的堆上的数据共享。这个 Arc 的引用计数被递增，以便将其传递给线程。
        joinhandles.push(thread::spawn(move || {  // 启动一个新线程并将其推入 joinhandles 中。这个线程会获取 child_numbers 的所有权。
            // 计算偏移量所对应的索引，然后将每个第 8 个值相加。
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap(); // d等待所有线程完成,如果没有这行代码，main 函数将会先于其他线程完成并退出，从而导致其他线程的工作被中断。
    }
}
