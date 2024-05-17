use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    // todo!("Count of occurrences of words in {words:?}");
    // 1. 先把字符串拆开
    let words: Vec<_> = words.split_whitespace().collect();
    let mut my_hash: HashMap<String, u32>  = HashMap::new();
    for word in words {
        let count = my_hash.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    
    return my_hash;
    // 2. 然后将字符串放入哈希表
}
