// 只引用到文件级别
use hello::my_add;
fn main() {
    println!("result = {}", my_add::add(13, 44));    
}
