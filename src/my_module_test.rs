// my_module_test.rs
mod my_module;
#[cfg(test)]
mod tests {
    use crate::my_module;
    #[test]
    fn test_add() {
        // 调用被测试的函数
        assert_eq!(my_module::add(2, 3), 5);
        assert_eq!(my_module::add(-1, 1), 0);
        assert_eq!(my_module::add(0, 0), 0);
    }
}
