extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::{thread, time::Duration};

fn main() {
    trpl::run(async {

        let a = async {
            println!("starting a");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("finished a");
        };

        let b = async {
            println!("starting b");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("finished b");
        };

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

// Listing 17-25: Using yield_now to let operations switch off making progress
