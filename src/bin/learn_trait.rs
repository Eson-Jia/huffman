
fn largest<T:Copy+PartialOrd>(list:&[T])-> T{
    let mut largest  = list[0];
    for & item  in list.iter() {
        if item> largest{
            largest = item;
        }
    }
    largest
}

fn longest<'a>(x:&'a str,y:&'a str)-> &'a str{
    if x.len()> y.len(){
        x
    }else{
        y
    }
}


fn main(){
    let  int_list  = vec![1,2,3,4,5,6];
    let  the_largest = largest(&int_list);
    println!("the largest is {}",the_largest);
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1[..],string2);
    println!("The longest string is {}",result);
}