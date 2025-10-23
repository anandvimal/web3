use std::sync::mpsc;
// mpsc stands for multiple producer, single consumer!.

fn main() {
    let (tx, rx) = mpsc::channel();

}

