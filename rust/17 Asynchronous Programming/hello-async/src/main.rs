extern crate trpl; // required for mdbook test

use std::{pin::pin, time::Duration};
use trpl::{StreamExt, Stream, ReceiverStream};

fn main() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("Received: {message}"),
                Err(reason) => eprintln!("Program : {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}

// Listing 17-34: Using the StreamExt::timeout method to set a time limit on the items in a stream