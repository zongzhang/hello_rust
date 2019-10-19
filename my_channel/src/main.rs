use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//fn main() {
//    //mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
//    let (tx, rx) = mpsc::channel();
//
//    thread::spawn(move || {
//        let val = String::from("hi");
//        tx.send(val).unwrap();
////        println!("val is {}", val); 这是错的，没有所有权了
//    });
//
//    let received = rx.recv().unwrap();
//    println!("Got: {}", received);
//
//}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}