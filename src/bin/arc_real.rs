use std::sync::Arc;
use std::thread;

fn main() {
    let number = Arc::new(10);

    let mut handles = vec![];

    for i in 0..3 {
        let num_clone = Arc::clone(&number);

        let handle = thread::spawn(move || {
            println!("Thread {:? } sees: {}",i, num_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}