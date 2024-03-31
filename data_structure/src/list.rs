pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
/*
1. 定义结构体类型
2. 实现结构体核心函数
3. 创建实例
*/

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}



fn main(){
    let mut lst1 = ListNode::new(1);
    let mut lst2 = ListNode::new(2);
    let mut lst3 = ListNode::new(3);
    // 插入
    lst2.next = Some(Box::new(lst3));
    lst1.next = Some(Box::new(lst2));
    let head = Some(Box::new(lst1));
    // 遍历
    // 没有实现`Copy`trait就是移动，移动完之后就不能再操作原来的值了。
    let mut cur = head;
    while let Some(node) = cur{
        println!("value = {}", node.val);
        cur = node.next;
    }
}
