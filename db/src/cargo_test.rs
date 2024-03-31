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
