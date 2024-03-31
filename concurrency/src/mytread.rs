fn main(){
    // 创建线程时的move是什么含义？move的是什么内容？
    // move 关键字的含义是告诉编译器闭包需要获取其使用的变量的所有权，而不是简单地通过引用来访问它们。
    // 如果没有传move，就表示是以引用的方式使用外部的变量。
    let mut x = 100;
    let mut v = vec![10, 20, 30];
    let mut v2 = vec![199, 299];
    // 在闭包内部被使用到的才会被move，没有被使用的，没有被move
    let handle = std::thread::spawn(move||{
        std::thread::sleep(std::time::Duration::from_secs(2));
        v[2] = 30000;
        println!("create thread A! v = {:?}", v);
    });

    let handle2 = std::thread::spawn(move||{
        println!("create thread B! x = {}", x+20);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("value = {}", x); // 但是普通类型i32，可以拷贝，所以move不影响
    println!("v = {:?}", v2); //外部不能再使用v变量了，因为已经移动到线程内容了。
}