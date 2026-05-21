use std::sync::Arc;

fn main() {
    // create a new Arc data type
    let a = Arc::new(5);
    println!("a is {:?}", a);//a is 5

    // clone a and the counter is increased by 1
    let b = Arc::clone(&a);
    println!("b is {:?}",b);// b is 5

    //display the counter which is 2 for this time
    println!("The counter is :{:?}",Arc::strong_count(&a));

}