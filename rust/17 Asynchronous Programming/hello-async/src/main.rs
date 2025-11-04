extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::{thread, time::Duration};

fn main() {
    trpl::run(async {

        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

// Listing 17-27: Using our imagined timeout to run a slow operation with a time limit.
