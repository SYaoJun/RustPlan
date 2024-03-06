use db;
#[test]
fn test_(){
    let s = "hello world".to_string();
    assert_eq!(db::last_word(s), 5);
}
/*
cargo test --test filename  其中filename不加rs
src/main.rs 中只会有几行代码-->调用src/lib.rs里的方法
单元测试 + 集成测试，核心代码放在src/lib.rs
基准测试：
冒烟测试
压力测试
rust性能好：安全（智能编译器）
这种我们能够聊一下吗？
*/