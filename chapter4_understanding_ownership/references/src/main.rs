fn calculate_length(s: &String) -> usize { // s os a reference to a String
    s.len()
} // s goes out of scope but s does not have ownership of the String so the String is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s gets dropped here so the reference will point to invalid data

// idiomatic Rust is for functions to not take ownership of their arguments unless they need to
// so we take a reference to a String
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert the String into an array of bytes

    // create an iterator over the array of bytes. enumerate() returns a tuple of the index and a
    // reference to the value at that index
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference that refers to the value of s1

    println!("the length of {s1} is {len}");

    let mut s2 = String::from("Hello");
    println!("s2 before: {s2}");
    change(&mut s2);

    println!("s2 after change: {s2}");

    let mut s = String::from("SomeString");
    let r1 = &mut s;
    println!("{r1}");
    let _r2 = &mut s;

    let reference_to_nothing = dangle();
}


