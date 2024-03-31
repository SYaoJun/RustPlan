
trait read_write {
    // 函数
    fn read(&self) -> String;
    fn write(&self)->bool;
}
// struct一般是大写
struct User{
    name: String,
    id: i32,
}
// 如果为某个trait实现函数，必须全部实现，不能加入其他的函数
impl read_write for User{
    fn read(&self)->String{
        println!("read success");
        return "hello".to_string();
    }
    fn write(&self)->bool{
        println!("write success");
        return true;
    }
}

impl User {
    fn new() -> User {
        User{  
            name: "yaojun".to_string(),
            id:10086,
        }
    }
}
fn main(){
    let user = User::new();
    println!("name = {}", user.name);
    user.read();
    user.write();
}