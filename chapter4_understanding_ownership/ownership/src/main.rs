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
}
