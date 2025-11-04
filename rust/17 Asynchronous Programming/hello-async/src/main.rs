extern crate trpl; // required for mdbook test

use std::pin::Pin;
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx1, mut rx) = trpl::channel();
        let tx2 = tx1.clone();

        let tx1_fut = pin!(async move {
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

        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx2_fut = pin!(async move {
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
        });

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(tx2_fut), Box::pin(rx_fut)];
        trpl::join_all(futures).await;     
    });
}

// Listing 17-18: Using Pin and Box::pin to make the Vec type check.
// Next we update the type annotation for futures, with a Pin wrapping each Box. Finally, we use Box::pin to pin the futures themselves.