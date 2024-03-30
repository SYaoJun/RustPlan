fn main(){
    let mut a: Option<i32> = None;
    // 存入
    a = Some(18);
    // 取出
    match a {
         Some(x)=> {
            println!("value = {}", x);
         }, 
        None =>{
            println!("None");
        } 
    }
}