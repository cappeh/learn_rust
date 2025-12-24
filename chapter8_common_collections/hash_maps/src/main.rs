use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:?}");

    let blue_team_name = String::from("Blue");
    // copied() takes Option<&i32> and returns Option<i32>
    // gets T from &T
    let blue_score = scores.get(&blue_team_name).copied().unwrap_or(0);
    println!("{blue_score}");

    let _yellow_team_name = String::from("Yellow");
    
    // the below returns Option<&i32> so unwrap_or would also need a reference
    let yellow_score = scores.get("Yellow").unwrap_or(&0);
    println!("{yellow_score:?}");

    // this will overwrite the value associated with the key "Yellow"
    scores.insert(String::from("Yellow"), 9);


    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
