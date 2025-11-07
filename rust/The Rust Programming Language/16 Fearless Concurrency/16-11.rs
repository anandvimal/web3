use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // Create another transmitter by cloning the first one. 
    // Now we have two transmitters: tx and tx1. use tx1 in one thread and tx in another.

    thread::spawn(move || {
        let vals = vec![
            String::from("hi 1"),
            String::from("from 1"),
            String::from("the 1"),
            String::from("thread 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("hi 2"),
            String::from("from 2"),
            String::from("the 2"),
            String::from("thread 2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
