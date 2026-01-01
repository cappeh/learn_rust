pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read More...)")
    }
}

pub trait SummaryAuthor {
    fn summarize_author(&self) -> String;
    fn summarize_2(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

impl SummaryAuthor for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryAuthor for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
