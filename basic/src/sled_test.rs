use sled::open;

#[allow(deprecated)]
fn main() {
    // 打开或创建一个名为 "my_database" 的数据库
    let db = open("my_database").unwrap();

    // 插入键值对
    db.insert(b"key1", b"value1").unwrap();

    // 获取键对应的值
    if let Some(value) = db.get(b"key1").unwrap() {
        println!("Value for key1: {:?}", value);
    } else {
        println!("Key1 not found");
    }

    // 更新键对应的值
    db.insert(b"key1", b"new_value1").unwrap();

    // 删除键值对
    db.remove(b"key1").unwrap();

    // 关闭数据库
    db.flush().unwrap();
}
