// Ownership Rules
// 1. Each value has an owner
// 2. There can only be one onwer at a time
// 3. Once the owner goes out of scope the value will be dropped


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

    let x = 5;
    let _y = x; // the value of x is copied into y

    let s1 = String::from("hello");
    let _s2 = s1; // s1 is no longer valid from this point

    // this println line would cause an error "value borrowed here after move" as it is no longer
    // valid
    // println!("{s1}, world");
}
