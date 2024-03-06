
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T>{
    size: usize,
    head: Link<T>,
}

pub struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl<T> List<T>{
    fn new()->Self{
        Self{
            size: 0,
            head: None
        }
    }
    fn is_empty(&self)->bool{
        self.size == 0
    }
    
    fn push(&mut self, elem: T){
        let node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }
    fn pop(&mut self)->Option<T>{
        if self.head.is_none(){
            return None;
        }
        self.head.take().map(|node|{
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }
}

fn main(){
    let mut lst = List::new();
    lst.push(1);
    lst.push(2);
    lst.push(3);
    assert_eq!(lst.pop(), Some(3));
    assert_eq!(lst.pop(), Some(2));
    assert_eq!(lst.is_empty(), false);
}

