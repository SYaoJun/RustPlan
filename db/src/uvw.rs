pub fn sum(a: i8, b: i8)->i8{
    a + b
}
/*
1. vector/string/hashmap
2. trait
3. box
4. unsafe
5. match/option
6. Rc/RefCell/Box/Arc
*/
// 手动引入
use std::collections::HashMap;
pub fn hash(){
    // 创建
    let mut mp: HashMap<i32, i32> = HashMap::new();
    // 插入
    // copy trait
    // 传引用
    mp.insert(10, 100);

    for (k, v) in &mp{
        print!("{}:{}", k,v);
    }

}
/*
单元测试
cargo test -- --ignored
集成测试：黑盒测试。tests目录。集成测试 位于 被测试代码的 外面
目的：测试被测试代码的多个方法是否能一起正确地工作
*/
#[cfg(test)]
mod tests{
    #[test]
    #[ignore]
    fn it_works(){
        assert_eq!(crate::sum(1, 8), 9);
    }
    /*测试一些期望抛出异常的代码*/
    #[test]
    #[should_panic]
    fn it_works_1(){
        assert_eq!(crate::sum(1, 8), 9);
    }
}

pub fn last_word(s: String)->i32{
    if s.trim().len() == 0 {
            return 0
        }
        let x = s.split_whitespace();
        for y in x{
            print!("len = {}\n", y.len());
        }
        9
}

pub fn to_lower_case(s: String) -> String {
    let mut ss = String::from("");
    for c in s.bytes(){
        if c >= b'A' && c <= b'Z'{
            ss.push((c+32) as char);
        }else{
            ss.push(c as char);
        }
    }
    ss
}




pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::from(digits);
    let mut c = 1;
    for mut x in (0..res.len()).rev(){
        c += x;
        x = c % 10;
        c /= 10;
    }
    if c != 0{
        res.insert(0, 1);
    }
    res
}