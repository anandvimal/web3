pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

// Listing 18-12: Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct

#[cfg(test)]
mod tests {
    use super::*;

}
