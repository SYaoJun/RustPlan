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
    // let mut v = vec![3,2,2,3];
    // let mut res = remove_element(&mut v, 3);
    // println!("res = {}", res);
    // vector_test();
    // vector_iterator();
    // let s = "hello world".to_string();
    // db::last_word(s);
    let s = "aaAHHHaa".to_string();
    println!("res = {}",db::to_lower_case(s));
}

pub fn count_key_changes(s: String) -> i32 {
    if s.is_empty(){
        return 0;
    }
    let mut last:u8 = 0;
    let mut cnt: i32 = 0;
    for c in s.as_bytes(){
        let mut x: u8 = 0;
        if *c >= b'a' && *c <= b'z'{
            x = *c-32;
        }else{
            x = *c;
        }
        if last != x {
            cnt+=1;
            last = x;
        }
    }
    cnt - 1
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // 遍历vector的方法
    let mut idx:i32 = 0;
    for i in 0..nums.len(){
        if nums[i] != val{
            nums[idx as usize] = nums[i];
            idx += 1;
        }
    }
    idx
}

pub fn vector_test(){
    let mut v = vec![100, 200, 400, 88];
    v.swap(0, 3);
    println!("{:?}", v);

}

pub fn vector_iterator(){

    let mut v = vec![100, 32, 57];
    check(&mut v);
    
    println!("{:?}", v);
}
fn check(v: &mut Vec<i32>){
    for x in v {
        *x += 100;
    }
}
