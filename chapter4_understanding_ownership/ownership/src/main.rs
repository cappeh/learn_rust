// Ownership Rules
// 1. Each value has an owner
// 2. There can only be one onwer at a time
// 3. Once the owner goes out of scope the value will be dropped

// this is because of line 25. Rust is telling us that we assigned "Hello" and never used it before
// reassigning the value
#[allow(unused_assignments)]
fn main() {
    // creating a new scope in main
    { // s is not valid yet as it is not declared
        let s = "hello"; // s is valid from this point onwards
        println!("{s}"); // make use of s in its current scope
    } // the scope ends and s is no longer valid

    // the ::from is a namespaced function (associative?)
    // this will allocate memory on the heap
    let mut s = String::from("Hello");
    s.push_str(", World"); // the String type is mutable
    println!("{s}");

    // integers like the below implement the Copy trait. (a type that has a known size at compile
    // time). variables that implement this type wont move, they are copied making them valid after
    // assignment to another variable. Types stored entirely on the stack
    let x = 5;
    let _y = x; // the value of x is copied into y

    let s1 = String::from("hello");
    // let _s2 = s1; // s1 is no longer valid from this point
    
    let s2 = s1.clone(); // performs a deep copy of the heap data and stack data
    println!("s1 = {s1}, s2 = {s2}");

    // this println line would cause an error "value borrowed here after move" as it is no longer
    // valid
    // println!("{s1}, world");
    
    let mut s3 = String::from("Hello");
    s3 = String::from("Ahoy"); // drop will be called on the initial value "hello" to free memory
    println!("{s3}, World!");
}
