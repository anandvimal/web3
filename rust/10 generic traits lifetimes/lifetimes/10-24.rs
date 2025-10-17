struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    println!("First sentence: {}", first_sentence);

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Important excerpt: {}", i.part);
}