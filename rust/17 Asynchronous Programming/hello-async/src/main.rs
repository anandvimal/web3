use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
    .select_first("title")
    .map(|title| title.inner_html())
}

// this fails to compile because main is not async
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    march page_title(url) {
        Some(title) => println!("The title for {url} is {title}"),
        None => println!("{url} had no title"),
    }
}
