pub fn reverse(input: &str) -> String {
    // 字符和字节还是有区别的，“子猫”是两个字符，如果纯看input的长度，那么n是6，所以要先转换为字符的长度
    let mut res: Vec<char> = input.chars().collect();
    let n = res.len();
    if n == 0 {
        return input.to_string();
    }

    let mut i = 0;
    let mut j = n - 1;
    println!("n = {}", n);
    while i < j {
        let c = res[i];
        res[i] = res[j];
        res[j] = c;

        i += 1;
        j -= 1;
    }

    res.into_iter().collect()
}

fn main() {
    println!("{}", reverse("子猫")); // 输出："猫子"
}
