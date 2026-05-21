// use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 1st way
    let a = RefCell::new(5);
    println!("a is {:?}",a);// a is RefCell { value: 5 }

    *a.borrow_mut()=7;
    println!("*a.borrow_mut() is {:?}",*a.borrow_mut());// *a.borrow_mut() is 7

    println!("*a.borrow() is {:?}",*a.borrow());// *a.borrow() is 7

    //2nd way
    let b=RefCell::new(10);

    println!("b is {:?}",b);// b is RefCell { value: 10}
    let c=b.borrow();
    println!("b.borrow() is {:?}",c);// b.borrow() is 10

    // we have to drop c , otherwise the program
    // will panic during run time
    drop(c);

    let d=b.borrow_mut();
    println!("b.borrow_mut is {:?}",d);//b.borrow_mut() is 10
}