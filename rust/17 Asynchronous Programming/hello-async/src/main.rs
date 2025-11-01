extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx1, mut rx) = trpl::channel();
        let tx2 = tx1.clone();

        let tx1_fut = async move {
            let vals = vec![
                String::from("1hi"),
                String::from("1from"),
                String::from("1the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }

        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx2_fut = async move {
            let vals = vec![
                String::from("2more"),
                String::from("2messages"),
                String::from("2for"),
                String::from("2you"),
            ];

            for val in vals {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx2_fut, rx_fut).await;
    });
}

// In Listing 17-13, 
// The key is the order in which the futures are awaited, not in which they’re created.
