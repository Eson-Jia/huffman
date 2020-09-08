use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut  the_heap =BinaryHeap::new();
    the_heap.push(Reverse(1));
    the_heap.push(Reverse(2));
    the_heap.push(Reverse(3));
    while the_heap.len()>0 {
        println!("{:?}",the_heap.pop());
    }
}
