use std::thread;

fn main() {
    // 创建一个新线程
    let handle = thread::spawn(|| {
        // 在新线程中执行的代码
        for i in 1..=5 {
            println!("Thread: {}", i);
        }
    });

    // 在主线程中执行的代码
    for i in 1..=3 {
        println!("Main: {}", i);
    }

    // 等待新线程结束
    handle.join().unwrap();

    // 主线程结束
}
