use std::rc::Rc;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

#[test]
fn test_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}


#[test]
fn test_multi_thread_multi_ownership() {
    let counter = Rc::new(Mutex::new(0));

//    let mut handles = vec![];
//
//    for _ in 0..10 {
//        let counter = Rc::clone(&counter);
//
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//            *num += 1;
//        });
//
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
}

#[test]
fn test_arc() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

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