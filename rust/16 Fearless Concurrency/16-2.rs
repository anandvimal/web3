use std::thread;
use std::time::Duration;

fn main() {

    // the return type of thread::spawn is JoinHandle<T>. We can store it in a variable to keep the spawned thread alive until we explicitly join it or let it go out of scope.
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("hi numer {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Wait for the spawned thread to finish
}