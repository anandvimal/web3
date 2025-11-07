// slight changes from book version. But works as same.
pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }
    
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!(" @{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
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

    notify(&article);
    notify(&post);
}