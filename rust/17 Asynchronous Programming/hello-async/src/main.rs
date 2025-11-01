extern crate trpl; // required for mdbook test

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received: {}", received);

    });
}

// listing 17-9