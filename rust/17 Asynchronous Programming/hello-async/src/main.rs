extern crate trpl; // required for mdbook test

//use std::pin::{Pin, pin};
use std::{thread, time::Duration};

fn main() {
    trpl::run(async {

        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        trpl::Either::Left(output) => Ok(output),
        trpl::Either::Right(()) => Err(max_time),
    }
}

// Listing 17-29: Defining timeout with race and sleep
