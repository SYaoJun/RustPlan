use std::io;

fn main() {
    // 提示用户输入多个数字
    println!("Please enter multiple integers separated by spaces:");

    // 创建一个可变的字符串变量，用于存储用户输入的一行文本
    let mut input = String::new();

    // 从标准输入中读取一行，并将结果存储到 input 变量中
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // 使用 split_whitespace 方法将字符串拆分为单独的数字
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // 打印用户输入的数字
    println!("You entered: {:?}", numbers);
}
