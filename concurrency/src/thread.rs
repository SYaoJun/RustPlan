use std::sync::{Arc, Mutex};
use std::thread;
/*
Arc<Mutex<List>>
这个可以理解为对List这个数据结构上锁吗？在并发场景下只有一个线程可以串行访问这个数结构？ 
*/
fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter2 = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
