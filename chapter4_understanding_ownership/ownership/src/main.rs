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

    let s4 = String::from("Some String"); // comes into scope
    takes_ownership(s4); // s4 moves into takes_ownership and no longer available in this scope
    // println!("{s4}"); this causes an error because s4 has moved to takes_ownership and no longer
    // available in this scope

    makes_copy(x); // i32 implements the copy trait so does not move into makes_copy
    println!("{x} from the main function"); // x can be used afterwards because of the Copy trait

    let _s5 = gives_ownership(); // return value from gives_ownership is moved into _s5
    let s6 = String::from("hello"); // comes into scope
    let s7 = takes_and_gives_back(s6); // s6 is moved into this function and its return value is
                                        // moved to _s7

    let (_s8, _len) = calculates_length(s7);
}

fn takes_ownership(some_string: String) { // some string comes into scope
    println!("{some_string}");
} // some_string goes out of scope and Drop is called and memory is freed

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String { // some_string is moved into the calling function
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope and is moved to the calling function
    a_string 
}

fn calculates_length(s: String) -> (String, usize) { // returns a tuple of String and usize
    let length = s.len();
    (s, length) // returns the string and length of the string
}
