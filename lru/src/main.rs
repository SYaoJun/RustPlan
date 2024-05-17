use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
type FrameId = u32;

struct LRUReplacer {
    total_pages: usize,
    // 双端队列
    lru_list: VecDeque<FrameId>,
    // 哈希表 key frequent
    lru_map: HashMap<FrameId, usize>,
    latch: Mutex<()>,
}

impl LRUReplacer {
    fn new(num_pages: usize) -> Self {
        Self {
            total_pages: num_pages,
            lru_list: VecDeque::new(),
            lru_map: HashMap::new(),
            latch: Mutex::new(()),
        }
    }

    fn victim(&mut self) -> Option<FrameId> {
        let _guard = self.latch.lock().unwrap();
        if self.lru_list.is_empty(){
            return None;
        }
        if let Some(&frame_id) = self.lru_list.back() {
            self.lru_list.pop_back();
            self.lru_map.remove(&frame_id);
            Some(frame_id)
        } else {
            None
        }
    }

    fn pin(&mut self, frame_id: FrameId) {
        // pin了之后，就需要从哈希表中移除，哈希表的key是frame_id，value是在双端队列中的下标
        // 从双端链表中移除
        let _guard = self.latch.lock().unwrap();
        if let Some(index) = self.lru_map.remove(&frame_id) {
            self.lru_list.remove(index);
            // 如果索引比index大 则减一
            for (_, idx) in &mut self.lru_map {
                if *idx > index {
                    *idx -= 1;
                }
            }
        }else{
            // println!("the frame_id = {}, not in the list", frame_id);
        }
    }

    fn unpin(&mut self, frame_id: FrameId) {
        let _guard = self.latch.lock().unwrap();
        // 满了 或者已经存在了 就不用unpin了 unpin就是加入哈希表
        if self.lru_map.len() >= self.total_pages || self.lru_map.contains_key(&frame_id) {
            return;
        }
        // 放在链表最前面，用个哈希表，记录每个frame出现了几次，从这里面选择一个出现次数最少的？，
        // 每次访问时携带一个时间戳，从队列中选择时间戳最早的？
        // 哈希表key肯定是frame_id，value可以是访问的时间戳
        self.lru_list.push_front(frame_id);
        // 1. 先将哈希表中下标+1
        for (_, idx) in &mut self.lru_map {
            *idx += 1;
        }
        // 2. 再插入头节点的索引
        self.lru_map.insert(frame_id, 0);
    }

    fn size(&self) -> usize {
        let _guard = self.latch.lock().unwrap();
        self.lru_map.len()
    }
}

fn main() {
    let lru_replacer = Arc::new(Mutex::new(LRUReplacer::new(7)));

    {
        let mut lru_replacer = lru_replacer.lock().unwrap();
        lru_replacer.unpin(1);
        lru_replacer.unpin(2);
        lru_replacer.unpin(3);
        lru_replacer.unpin(4);
        lru_replacer.unpin(5);
        lru_replacer.unpin(6);
        lru_replacer.unpin(1); 
        // for i in 0..lru_replacer.lru_list.len() {
        //     println!("deque: index = {} , value = {}", i, lru_replacer.lru_list[i]);
        // }
        // for p in lru_replacer.lru_map.iter() {
        //     println!("key = {}, value = {}", p.0, p.1);
        // }
        assert_eq!(6, lru_replacer.size());
        // Scenario: get three victims from the lru.
        #[warn(unused_assignments)]
        let mut value:Option<u32> = Some(0);
        value = lru_replacer.victim();
        assert_eq!(Some(1), value);
        value = lru_replacer.victim();
        assert_eq!(Some(2), value);
        value = lru_replacer.victim();
        assert_eq!(Some(3), value);

        // Scenario: pin elements in the replacer.
        // Note that 3 has already been victimized, so pinning 3 should have no effect.
        // 3已经被淘汰了，所以pin 3应该没有影响
        lru_replacer.pin(3);
        // pin 4此时只有 5和6处于unpin，可以被淘汰
        // 6 5 4
        
        lru_replacer.pin(4);
        assert_eq!(2, lru_replacer.size());

        // Scenario: unpin 4. We expect that the reference bit of 4 will be set to 1.
        // 如果这个时候unpin 4，那么此时可以淘汰的有 4 5 6
        
        lru_replacer.unpin(4);

        // Scenario: continue looking for victims. We expect these victims.
        value = lru_replacer.victim();        
        assert_eq!(Some(5), value);
        value = lru_replacer.victim();
        assert_eq!(Some(6), value);
        value = lru_replacer.victim();
        assert_eq!(Some(4), value);
    }
}

// #[test]
// fn multi_thread_test() {
//     // 创建一个线程安全的 LRUReplacer 实例
//     let lru_replacer = Arc::new(Mutex::new(LRUReplacer::new(3)));

//     // 定义一个闭包来访问 LRUReplacer 实例，并执行一些操作
//     let access_task = {
//         let lru_replacer = Arc::clone(&lru_replacer);
//         move || {
//             let mut lru = lru_replacer.lock().unwrap();
//             lru.unpin(1);
//             lru.unpin(2);
//             lru.unpin(3);
//             println!("Size after unpinning: {}", lru.size());

//             if let Some(victim) = lru.victim() {
//                 println!("Victim: {:?}", victim);
//             } else {
//                 println!("No victim found");
//             }

//             lru.pin(2);
//             println!("Size after pinning: {}", lru.size());
//         }
//     };

//     // 创建多个线程来并发访问 LRUReplacer
//     let mut handles = vec![];
//     for _ in 0..5 {
//         let handle = thread::spawn(access_task.clone());
//         handles.push(handle);
//     }

//     // 等待所有线程完成
//     for handle in handles {
//         handle.join().unwrap();
//     }
// }
