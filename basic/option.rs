fn main(){
    let mut a: Option<i32> = None;
    // 存入
    a = Some(18);
    // 取出
    if let Some(x) = a{
        println!("value = {}", x);
    }
}