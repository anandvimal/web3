extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("async"),
            String::from("world!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

// listing 17-10
// reciever awaits messages from a channel and does not exit until the channel is closed
