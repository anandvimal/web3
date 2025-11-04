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

        let futures: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(tx2_fut), Box::new(rx_fut)];
        trpl::join_all(futures).await;     
    });
}

// Listing 17-17: Fixing the rest of the type mismatch errors by using an explicit type declaration
// We’ll come back to the Unpin errors in a moment. 
// First, let’s fix the type errors on the Box::new calls by explicitly annotating the type of the futures variable