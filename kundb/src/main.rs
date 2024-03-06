// pub mod orset;
use std::collections::HashMap;

use kundb::check_x_matrix;
struct Point {
    length: i32,
    high: i32,
    name: String,
}
fn main() {
    // let x:Box<Point> = Box::new(Point{
    //     length: 10, high: 20, name: "hello".to_string()
    // });
    // take_box(x);
    // let v  = vec![1,4,6,9];
    // let res = check(v);
    // println!("res = {}", res);
    // option_test();
    // copy_test();
    // let re = kundb::minimum_operations(v);
    // let res = kundb::count_asterisks("l|*e*et|c**o|*de|".to_string());
    let v: Vec<Vec<i32>> = vec![vec![2,0,0,1],vec![0,3,1,0],vec![0,5,2,0],vec![4,0,0,2]];
    let res = kundb::check_x_matrix(v);
        println!("res = {}", res);
}

 fn take_box(b: Box<Point>){
    let len = b.length;
    println!("length = {}", len);
 }



 fn check(v: Vec<i32>)->bool{
    let mut mp: HashMap<i32, i32> = HashMap::new();
    for key in v{
        mp.entry(key).and_modify(|v| *v += 1).or_insert(1);
    }
    for v in mp.values(){
        if v > &1{
            return false;
        }
    }
    return true;
 }

 fn option_test(){
    // Option是枚举类型
    // Result也是枚举类型
    // Some是大写
    let o1: Option<i32> = Some(128);
    let x = 128;
    // if let Some(x) = o1{
    //     println!("ok {}", x);
    // }
    match o1{
        Some(x)=>{
            println!("some");
        },
        None=>{
            println!("None");
        }
    }


 }

 struct Number {
    odd: bool,
    value: i32,
}
// borrow
fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn copy_test(){
    let n = Number { odd: true, value: 51 };
    print_number(&n); 
    print_number(&n); 
}

