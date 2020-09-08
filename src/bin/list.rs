use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::{File};
use std::io::{Read};


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

fn build_code(root:&Node)->HashMap<char,String>{
    let mut map = HashMap::new();
    re_build_code(root,&mut map,"");
    map
}

fn re_build_code(node:&Node,map:&mut HashMap<char,String>,prefix:&str){
   if node.is_leaf() {
       map.insert(node.inner,String::from(prefix));
       return;
   }
    if let Some(left) = node.left.as_ref(){
        let left = left.as_ref();
        let mut prefix = prefix.to_string();
        prefix.push_str("0");
        re_build_code(left,map,&prefix[..]);
    }else{
        panic!("left node is None !");
    }
    if let Some(right) = node.right.as_ref(){
        let right = right.as_ref();
        let mut prefix = prefix.to_string();
        prefix.push_str("1");
        re_build_code(right,map,&prefix[..]);
    }else {
        panic!("right node is None !");
    }
}

fn main(){
    let mut f  = File::open("./text.txt").expect("open text.txt failed");
    let mut text = String::new();
    f.read_to_string(& mut text).unwrap();
    let  root  =  build_trie(build_forest(&text[..]));
    if let Some(root) = root{
        for (k,v) in build_code(&root){
            println!("{} --> {}",k,v);
        }
    };
}

