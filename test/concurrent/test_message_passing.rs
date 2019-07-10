use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_message_passing() {
    let (tx, rx) = mpsc::channel();
    tx.send("123").unwrap();
    for i in rx.recv() {
        println!("{}", i);
    }
}

#[test]
fn test_message_passing_in_thread() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
fn test_message_passing_ownership_transfer() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = "hi".to_owned();
        tx.send(val).unwrap();
//        println!("val is {}", val);
    });

    let recv = rx.recv().unwrap();
    println!("Got: {}", recv);
}

#[test]
fn test_message_passing_multi_value() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            "hi".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("Got: {}", recv);
    }
}

#[test]
fn test_multi_producer() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move|| {
        let vals = vec![
            "hi".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned(),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "more".to_owned(),
            "messages".to_owned(),
            "for".to_owned(),
            "you".to_owned(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("Got: {}", recv);
    }

}
