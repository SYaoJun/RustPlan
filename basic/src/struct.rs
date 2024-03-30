/*定义结构体*/
#[derive(Debug)]
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
