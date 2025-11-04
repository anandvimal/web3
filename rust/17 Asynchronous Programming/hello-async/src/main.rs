extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        let a = async {
            println!("starting a");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            println!("finished a");
        };

        let b = async {
            println!("starting b");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("finished b");
        };

        trpl::race(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

// Listing 17-23: Using thread::sleep to simulate slow operations
