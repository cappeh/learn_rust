fn calculate_length(s: &String) -> usize { // s os a reference to a String
    s.len()
} // s goes out of scope but s does not have ownership of the String so the String is not dropped

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference that refers to the value of s1

    println!("the length of {s1} is {len}");

    let mut s2 = String::from("Hello");
    println!("s2 before: {s2}");
    change(&mut s2);

    println!("s2 after change: {s2}");
}
