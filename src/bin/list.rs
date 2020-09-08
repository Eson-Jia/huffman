use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Clone, Eq, PartialEq,Debug)]
pub struct Node {
    pub inner:char,
    pub frequent:u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl  Node {
    #[inline]
  pub fn new(inner:char,freq:u32)->Self{
        Node{
            inner,
            frequent: 0,
            left: None,
            right: None,
        }
    }
}


impl  Ord for Node {
    fn cmp(&self,other:&Node)->Ordering{
        other.frequent.cmp(&self.frequent)
    }
}

 impl PartialOrd for Node {
     fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
         Some(self.cmp(other))
     }
 }

fn main(){
    // let mut head = Node::new('d',123);
    // head.left = Some(Box::new(Node::new('b',21)));
    let mut the_heap = BinaryHeap::new();
    for i in 0..26 {
        match std::char::from_u32(97+i) {
            Some(char)=>the_heap.push(Node::new(char,i+1)),
            None=> panic!("isn't a ascii"),
        }
    }
    while the_heap.len()>1{
        let  first = the_heap.pop();
        let  second = the_heap.pop();
        match first {
            Some(first)=> match second {
                Some(second)=>{
                    let mut parent =Node::new('\0',first.frequent+second.frequent);
                    // 在这里将 first move 到 Box 里面并设置到父节点的left
                    parent.left=Some(Box::from(first));
                    // 取消下面的注释会报错: value borrowed here after move
                    // println!("the first is {:?}",first);
                    parent.right=Some(Box::from(second));
                    the_heap.push(parent);
                },
                None=>panic!("second is None"),
            },
            None=>panic!("first is None"),
        }
    };
    if let Some(root) =  the_heap.pop() {
        // println!("{:?}",root);
       let result= get_last(&root);
        println!("{:?}",result);
        let result= get_last(&root);
        println!("{:?}",result);
    };
}



pub fn get_last(root: &Node)->&Node {
    let mut lhs = root;
    while let Some( node) = lhs.left.as_ref(){
        lhs = node.as_ref();
    }
    lhs
}

