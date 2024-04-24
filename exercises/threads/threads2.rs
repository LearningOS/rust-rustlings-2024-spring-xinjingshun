// threads2.rs
//
// 在上一个练习的基础上，我们希望所有线程都完成其工作，但这次生成的线程需要负责更新一个
// 共享值：JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::sync::{Arc,Mutex}; //添加互斥锁
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc<Mutex<JobStatus>> 来包装 JobStatus，以便线程安全地共享和修改。
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: 在更新共享值之前，您必须采取一个操作。
            // 为了修改 jobs_completed，首先需要获取互斥锁的锁。
            // lock 方法会阻塞当前线程直到互斥锁可用。
            let mut num = status_shared.lock().unwrap();
            num.jobs_completed += 1;
            println!("jobs_completed is {:?}", num.jobs_completed)
            // 锁在 num 离开作用域时自动释放。
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: 打印 JobStatus.jobs_completed 的值。你有没有在输出中注意到什么有趣的内容？你是否需要在所有的句柄上进行 'join' 操作？
        // 由于我们使用了互斥锁，所有线程对 jobs_completed 的增加都会是线程安全的。
        // 下面是不加锁打印，打印的结果很可能不递增，出现连续打印相同值
        // println!("jobs completed {:?}", status.lock().unwrap().jobs_completed);
        // 下面是加锁打印，将只打印出最终结果10
        println!("jobs completed {}，and locked print", {
            let lock = status.lock().unwrap();
            lock.jobs_completed
        });
    }
}
