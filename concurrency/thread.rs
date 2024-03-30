use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个可变的全局变量，并将其放入互斥锁中
    let counter = Arc::new(Mutex::new(0));

    // 创建一组线程来增加计数器的值
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 获取互斥锁，以确保同时只有一个线程能够访问计数器
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终计数器的值
    println!("Final count: {}", *counter.lock().unwrap());
}
