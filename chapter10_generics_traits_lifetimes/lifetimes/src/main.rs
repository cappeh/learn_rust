use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    // lifetime elision rule 1 applies here, only one reference parameter so the compiler will
    // assign a generic lifetime param
    fn level(&self) -> i32 {
        3
    }

    // lifetime elision rule 3 applies here, there are two parameters so each are assigned a
    // lifetime param. Because &self is one, the output reference (return type &str) is assigned
    // the same lifetime as &self
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention Please: {announcement}");
        self.part
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let x;

    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is: {result}");

        x = return_x(string1.as_str(), string2.as_str());
    }

    println!("x is: {x}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Novel First Line: {i:?}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}

fn return_x<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

#[allow(dead_code)]
// generics and lifetime params are used in the same list <>
// the generic type T must be a type that implements the Display trait (where)
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {x} else {y}
}


