extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            println!("slow started");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("slow finished");
        };

        let fast = async {
            println!("fast started");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("fast finished");
        };

        trpl::race(slow, fast).await;
    });
}

// Listing 17-21: Using race to get the result of whichever future finishes first
