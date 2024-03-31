fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];

    // 使用闭包将向量中的每个元素加倍并打印
    nums.iter_mut().for_each(|x| {
       *x *= 2; 
    });
    println!("nums = {:?}", nums);
}