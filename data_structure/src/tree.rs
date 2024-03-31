// 定义一个递归的数据结构，可以在编译时确定大小
enum BinaryTree {
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
    Leaf,
}

fn main() {
    // 创建一个递归的二叉树数据结构
    let tree = BinaryTree::Node(
        1,
        Box::new(BinaryTree::Node(2, Box::new(BinaryTree::Leaf), Box::new(BinaryTree::Leaf))),
        Box::new(BinaryTree::Leaf),
    );

    // 打印二叉树
    print_tree(&tree);
}

// 打印二叉树
fn print_tree(tree: &BinaryTree) {
    match tree {
        BinaryTree::Node(value, left, right) => {
            print_tree(left);
            println!("{}", value);
            print_tree(right);
        }
        BinaryTree::Leaf => {}
    }
}
