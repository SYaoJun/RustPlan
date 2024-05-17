use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;

const COUNT: u32 = 1000000;

fn main() {
    // Atomic 系列类型同样提供了线程安全版本的内部可变性
    let global = Arc::new(AtomicIsize::new(0));

    let clone1 = global.clone();
    let thread1 = thread::spawn(move|| {
        for _ in 0..COUNT {
            clone1.fetch_add(1, Ordering::SeqCst);
        }
    });

    let clone2 = global.clone();
    let thread2 = thread::spawn(move|| {
        for _ in 0..COUNT {
            clone2.fetch_sub(1, Ordering::SeqCst);
        }
    });

    thread1.join().ok();
    thread2.join().ok();
    println!("final value: {:?}", global);
}