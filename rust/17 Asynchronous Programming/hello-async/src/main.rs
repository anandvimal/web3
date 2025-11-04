extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::{thread, time::Duration};

fn main() {
    trpl::run(async {
        // we will call 'slow' here later.
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name} ran for {ms} ms");
}

// Listing 17-22: Using thread::sleep to simulate slow operations
