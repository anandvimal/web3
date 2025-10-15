// slight changes from book version. But works as same.
pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("John Doe"),
        content: String::from("The Penguins have won the Stanley Cup for the third time in six years."),
    };

    let post = SocialPost {
        username: String::from("user123"),
        content: String::from("Just watched the game, what a win!"),
        reply: false,
        retweet: true,
    };

    println!("New article available! {}", article.summarize());
    println!("New post: {}", post.summarize());
}