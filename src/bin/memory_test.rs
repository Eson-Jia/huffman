#![feature(slice_fill)]

use std::thread::sleep;
use std::time::Duration;

fn main(){
    let mut the_vec = Vec::new();
    let mut count =0;
    loop {
        sleep(Duration::new(1,0));
        the_vec.push(Vec::with_capacity(1024).fill(1024));
        count+=1;
        println!("use {:?} M",count);
    }
}