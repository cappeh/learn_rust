fn main() {
    // to create a new instance of a String
    let mut s = String::new();
    s.push_str("initial contents");
    println!("{s}");

    let some_data = "initial data";
    let s2 = some_data.to_string();
    println!("{s2}");

    let hello = String::from("Hello");
    let world = String::from("World");
    // this is one way of concatenating a string but "hello" is moved into the add function
    // used to concat the two strings
    // let hello_world = hello + &world;
    // println!("{hello_world}");

    let hello_world = format!("{hello} {world}");
    println!("{hello_world}");
}
