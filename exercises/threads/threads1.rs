// threads1.rs
//
// 这个程序生成多个线程，每个线程至少运行250毫秒，
// 并且每个线程返回它们完成所需的时间。程序应该等待所有生成的线程都完成，并将它们的返回值收集到一个向量中
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // 创建一个空的线程句柄向量，用于存储每个线程的句柄。
    let mut handles = vec![];

// 循环从0到9（不包括10），创建10个线程。
    for i in 0..10 {
        // 使用thread::spawn启动一个新的线程。
        // 每个线程的闭包参数使用move关键字，意味着闭包会获取捕获变量的所有权。
        handles.push(thread::spawn(move || {
            // 使用Instant::now()获取当前的精确时间，作为线程开始执行的时间点。
            let start = Instant::now();
            // 使用thread::sleep使线程休眠250毫秒。
            thread::sleep(Duration::from_millis(250));
            // 打印一条消息，表明当前的线程（由变量i表示）已经完成。
            println!("thread {} is complete", i);
            // 计算从线程开始到当前经过的时间，并以毫秒为单位返回。
            // start.elapsed()获取从start到当前时间的持续时间。
            // as_millis()将这个时间转换为毫秒。
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: 一个结构体从thread::spawn返回，你可以使用它吗？
        // 使用join等待线程完成并获取返回值
        match handle.join(){
            Ok(Duration) => results.push(Duration),
            Err(e) => println!("Error {:?}",e)
        }
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
