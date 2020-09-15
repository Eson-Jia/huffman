use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::{File};
use std::io::{Read, BufWriter, Write};


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

struct BinaryWriter {
    pub out:BufWriter<File>,
    pub buffer:u8,
    pub n:u32,
}

impl BinaryWriter{
    fn new(out:BufWriter<File>) ->Self{
        BinaryWriter{
            out,
            buffer:0,
            n:0,
        }
    }
}

impl BinaryWriter {
    fn clear_buffer(&mut self){
        if self.n==0{
            return;
        }
        if self.n>0 {
            self.buffer <<= (8 - self.n) as u8;
            self.out.write(&[self.buffer,1]).expect("failed in write buffer");
        }
        self.n = 0;
        self.buffer = 0;
    }

    fn write_bit(&mut self, bit:bool){
        // if bit{
        //     print!("1");
        // }else{
        //     print!("0");
        // }
        self.buffer<<=1;
        if bit {
            self.buffer |=1;
        }
        self.n+=1;
        if self.n ==8 {
            self.clear_buffer();
        }
    }

    fn write_byte(& mut self,byte:u8){
        if self.n ==0{
            self.out.write(&[byte,1]).expect("failed in write_byte");
            return;
        }
        for i in 0..8 {
            let bit = ((byte>>(8-i-1))&1) ==1;
            self.write_bit(bit);
        }
    }

    fn close(& mut self){
        self.flush();
    }

    fn flush(& mut self){
        self.clear_buffer();
        self.out.flush().expect("failed in out.flush");
    }
}

// write_trie 前序遍历
fn write_trie(node:&Node,binary:&mut BinaryWriter){
    if node.is_leaf(){
        binary.write_bit(true);
        if node.inner.is_ascii(){
            let  char_u8 = node.inner as u8;
            binary.write_byte(char_u8);
        }else{
            panic!("{} isn't a ascii",node.inner);
        }
        return;
    }
    binary.write_bit(false);
    let left = node.left.as_ref().expect("").as_ref();
    write_trie(left,binary);
    let right = node.right.as_ref().expect("").as_ref();
    write_trie(right,binary);
}


fn main(){
    let mut f  = File::open("/home/ubuntu/Documents/text.txt").expect("open text.txt failed");
    let mut text = String::new();
    let buf_writer = BufWriter::new(File::create("./text.bin").expect("failed in open file"));
    let mut binary_writer  = BinaryWriter::new(buf_writer);
    f.read_to_string(& mut text).unwrap();
    let  root =  build_trie(build_forest(&text[..])).expect("root is None");
    let prefix_code = build_code(&root);
    for (k,v) in &prefix_code{
        let k = *k;
        println!("{}/{} --> {}",k as u8,k,v);
    }
    write_trie(&root,&mut binary_writer);
    for char in text.chars(){
        let  prefix = prefix_code.get(&char).expect("char not found");
        for code in prefix.chars(){
            match code {
                '0'=>binary_writer.write_bit(false),
                '1'=>binary_writer.write_bit(true),
                _=>(),
            }
        }
    }
}

