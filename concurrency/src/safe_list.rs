use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::thread;

// 创建一个包含 Mutex 和 LinkedList 的结构体，用于表示线程安全的链表
struct ThreadSafeList<T> {
    inner: Arc<Mutex<LinkedList<T>>>,
}

impl<T: std::fmt::Debug> ThreadSafeList<T> {
    // 创建一个新的线程安全链表
    fn new() -> Self {
        ThreadSafeList {
            inner: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    // 插入元素到链表尾部
    fn push_back(&self, value: T) {
        let mut list = self.inner.lock().unwrap();
        list.push_back(value);
    }

    // 从链表尾部删除元素并返回它
    fn pop_back(&self) -> Option<T> {
        let mut list = self.inner.lock().unwrap();
        list.pop_back()
    }

    // 打印链表内容
    fn print(&self) {
        let list = self.inner.lock().unwrap();
        println!("List contents: {:?}", *list);
    }
}

fn main() {
    let list = Arc::new(ThreadSafeList::new());

    // 创建两个线程并行地向链表中插入元素
    let list_clone = Arc::clone(&list);
    let thread1 = thread::spawn(move || {
        list_clone.push_back(1);
        list_clone.push_back(2);
    });

    let list_clone = Arc::clone(&list);
    let thread2 = thread::spawn(move || {
        list_clone.push_back(3);
        list_clone.push_back(4);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // 打印链表内容
    list.print();

    // 从链表尾部删除元素
    let popped_value = list.pop_back();
    if let Some(value) = popped_value {
        println!("Popped value: {}", value);
    } else {
        println!("List is empty");
    }
}
