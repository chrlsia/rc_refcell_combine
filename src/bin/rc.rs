use std::rc::Rc;
// use std::cell::RefCell;

fn main() {
    let a = Rc::new(0);
    println!("a is {:?}",a); // a is 0

    let b = Rc::clone(&a);
    
    println!("b is {:?}",b); // b is 0

    println!("counter is :{:?}",Rc::strong_count(&a));// counter is :2

}