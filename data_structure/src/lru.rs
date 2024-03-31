use std::collections::{HashMap, LinkedList};
use std::sync::{Arc, Mutex};
use std::thread;
/*
LRU的核心API
1. access
2. evict
3. remove
4. 自己实现双向链表
*/
#[derive(Debug)]
struct LRUCache{
    capacity: usize,
    // 哈希表放置key和list_index
    cache: HashMap<i32, i32>,
    // 链表放置key
    lru_order: LinkedList<i32>,
    // rust链表中的删除和插入O(1)
    current_size: usize,
}

impl LRUCache{
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            lru_order: LinkedList::new(),
            current_size: 3,
        }
    }

    fn get(&mut self, key: &i32) -> Option<&i32> {
        // 将访问过的键移到链表头部表示最近访问
        // 1. 从链表中删除
        // 2. 加入到链表头
        // 链表头数据更新一些
        if let Some(&k) = self.lru_order.iter().find(|&&k| k == *key) {
            self.lru_order.remove(&k);
            self.lru_order.push_front(k);
            self.cache.get(key)
        } else {
            None
        }
    }

    fn insert(&mut self, key: i32, value: i32) {
        if self.cache.len() >= self.capacity {
            // 如果容量已满，移除最久未使用的键
            if let Some(oldest_key) = self.lru_order.pop_back() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, value);
        self.lru_order.push_front(key);
    }
    pub fn size(&self) -> usize {
        self.current_size
    }
}

fn main() {
    let cache = Arc::new(Mutex::new(LRUCache::new(3)));

    // 插入一些数据
    {
    
        let mut cache = cache.lock().unwrap();
        cache.insert(1, 100);
        cache.insert(2, 200);
        cache.insert(3, 300);
    }

    // 并发访问缓存数据
    let handles = (0..5)
        .map(|i| {
            let cache = Arc::clone(&cache);
            thread::spawn(move || {
                let mut cache = cache.lock().unwrap();
                match i {
                    0 => {
                        println!("Thread {}: {:?}", i, cache.get(&1));
                        println!("Thread {}: {:?}", i, cache.get(&2));
                    }
                    1 => {
                        println!("Thread {}: {:?}", i, cache.get(&2));
                        println!("Thread {}: {:?}", i, cache.get(&3));
                    }
                    2 => {
                        println!("Thread {}: {:?}", i, cache.get(&3));
                        println!("Thread {}: {:?}", i, cache.get(&1));
                    }
                    3 => {
                        cache.insert(4, 400);
                        println!("Thread {}: Inserted (4, \"Four\")", i);
                    }
                    4 => {
                        cache.insert(5, 500);
                        println!("Thread {}: Inserted (5, \"Five\")", i);
                    }
                    _ => unreachable!(),
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }
}
