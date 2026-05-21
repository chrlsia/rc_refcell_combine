use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // create a value 10
    // capable to mutate it
    // in an environemnt with mupliple ownership
    let a: Rc<RefCell<i32>>=Rc::new(RefCell::new(10));
    println!("a is {:?}",a);// a is RefCell { value: 10 }

    // create another ownership for a, module_a
    let module_a: Rc<RefCell<i32>>=Rc::clone(&a);
    // let's see the value of module_a
    println!("module_a is {:?} ",module_a);//module_a is RefCell { value: 10 }

    // create another ownership for a, modulo_b
    let module_b=Rc::clone(&a);
    // let's see the value of module_b
    println!("module_b is {:?} ",module_b);//module_b is RefCell { value: 10 }

    //let's increase the value of 10 by 2
    *module_a.borrow_mut()+=2;
    println!("*module_a.borrow() ={:?}",*module_a.borrow());//*module_a.borrow() =12
    //now the value is 12, let's icrease it by 3
    *module_b.borrow_mut()+=3;
    println!("*module_b.borrow() ={:?}",*module_b.borrow());//*module_b.borrow() =15

    //so what's the value of a? it should be 15
    println!("*a.borrow()={:?}",*a.borrow()); //a.borrow()=15



    
}