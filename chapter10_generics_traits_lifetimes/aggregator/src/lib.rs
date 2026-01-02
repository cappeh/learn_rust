// use std::fmt::{Debug, Display};

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

// this is syntax sugar for a trait bound set below
// pub fn notify(item: &impl Summary) {
//     println!("Breaking News! {}", item.summarize());
// }

// this is a trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

// multiple trait bounds
// pub fn notify(item: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item: &T) {}


// where clause allows a function signature to become less cluttered
// especially if types need to implement many traits
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where 
//     T: Display + Clone,
//     U: Clone + Debug,
// {}
