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
    
}

trait State {}

struct Draft {}

impl State for Draft {}

// Listing 18-13: Implementing the add_text method to add text to a post’s content

#[cfg(test)]
mod tests {
    use super::*;

}
