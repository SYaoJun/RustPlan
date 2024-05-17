use std::sync::{Arc, RwLock};
use std::thread;

const COUNT: u32 = 1000000;

fn main() {
    let global = Arc::new(RwLock::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone1.write().unwrap();
            *value += 1;
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move || {
        for _ in 0..COUNT {
            let mut value = clone2.write().unwrap();
            *value -= 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // 锁定 RwLock 获取内部的值
    let value = global.write().unwrap();
    println!("final value: {:?}", *value);
}
