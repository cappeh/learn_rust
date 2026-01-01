use aggregator::{NewsArticle, SocialPost, Summary, SummaryAuthor};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",
            ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    println!("Another new post: {}", post.summarize_2());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
            hockey team in the NHL",
            ),
    };

    println!("News Article available! {}", article.summarize());
    println!("Another article available: {}", article.summarize_2());
}
