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

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

// Listing 18-14: Adding a placeholder implementation for the content method on Post that always returns an empty string slice

#[cfg(test)]
mod tests {
    use super::*;

}
