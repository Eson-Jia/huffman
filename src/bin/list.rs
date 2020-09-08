use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};


#[derive(Clone, Eq, PartialEq,Debug)]
pub struct Node {
    inner:char,
    frequency:u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl  Node {
    #[inline]
  pub fn new(inner:char,frequency:u32)->Self{
        Node{
            inner,
            frequency,
            left: None,
            right: None,
        }
    }
}

impl  Ord for Node {
    fn cmp(&self,other:&Node)->Ordering{
        other.frequency.cmp(&self.frequency)
    }
}

 impl PartialOrd for Node {
     fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
         Some(self.cmp(other))
     }
 }

impl Node{
    fn is_leaf(&self)->bool{
        self.right.is_none()&& self.left.is_none()
    }
}

pub fn build_forest(text:&str)-> Vec<Node>{
    let mut counter = HashMap::new();
    for char in text.chars() {
        let count = counter.entry(char).or_insert(0);
        *count+=1;
    }
    let mut the_forest = Vec::new();
    for (char,freq) in counter {
        the_forest.push(Node::new(char, freq));
    }
    the_forest
}

pub fn build_trie(forest:Vec<Node>)->Option<Node> {
    let mut the_heap  =  BinaryHeap::from(forest);
    while the_heap.len()>1{
        let  first = the_heap.pop();
        let  second = the_heap.pop();
        match first {
            Some(first)=> match second {
                Some(second)=>{
                    let mut parent =Node::new('\0',first.frequency+second.frequency);
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
    the_heap.pop()
}

pub fn get_last(root: &Node)->&Node {
    let mut lhs = root;
    while let Some( node) = lhs.left.as_ref(){
        lhs = node.as_ref();
    }
    lhs
}

fn main(){
    let text = "this is the best of time,this is the worst of time";
    let  root  =  build_trie(build_forest(text));
    if let Some(root) = root{
        // println!("{:?}",root);
        let result= get_last(&root);
        println!("{:?}",result);
        let result= get_last(&root);
        println!("{:?}",result);
    };
}

