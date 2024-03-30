# 基础知识
- 变量
- 借用
- 生命周期
- 字符串
- 标准库(Vec,String,HashMap)
- Option
- 结构体
- 函数
- 实现结构体
- 智能指针
- 多线程
- 单元测试
- 代码格式化
- 文件读写
## 创建结构体
```sh
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    /*使用结构体*/
    let user = User {
        username: "yaojun".to_string(),
        email: "940334249@qq.com".to_string(),
        sign_in_count: 88,
        active: true,
    };
    println!("res = {:?}", user);
}
```
## Option的用法
```sh
fn main(){
    let mut a: Option<i32> = None;
    // 存入
    a = Some(18);
    // 取出
    if let Some(x) = a{
        println!("value = {}", x);
    }
}
```
## 编译指定二进制
```sh
 cargo run --bin option
```

