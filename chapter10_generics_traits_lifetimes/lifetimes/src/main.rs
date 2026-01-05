#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
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
