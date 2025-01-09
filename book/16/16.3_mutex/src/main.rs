// use std::rc::Rc;
// use std::sync::Mutex;
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter); // Atomic Reference Counting(Rc), Rc와 비슷하지만 스레드 간에 공유할 수 있다.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
