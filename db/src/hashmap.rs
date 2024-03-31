#[warn(unused_imports)]
use std::collections::HashMap;

#[test]
fn hash_test(){
    // 创建
    let mut mp: HashMap<i32, String> = HashMap::new();
    // 插入
    // copy trait
    // 传引用
    let s = "hello";
    mp.insert(10, s.to_string());

    for (k, v) in &mp{
        print!("{}:{}", k,v);
    }
    // 不存在就插入
    if mp.contains_key(&10) {
       println!("存在");
    }else{
       println!("不存在");
    }

    mp.entry(100).or_insert("fasdf".to_string());
    for p in mp.iter(){
       println!("{},{}", p.0, p.1);
    }
}
fn main(){
    hash_test();
}


