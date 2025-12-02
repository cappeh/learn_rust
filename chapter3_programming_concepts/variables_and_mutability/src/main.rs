fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    const MAX_POINTS: u32 = 1_800;
    println!("A user can get {MAX_POINTS} maximum points");

    // shadowing
    let y = 5;
    let y = y + 1; // y = 6

    {
        let y = y * 2; // y = 12 in this scope
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The lenght of spaces is: {spaces}");

    let mut mutable_binding = 1;
    println!("Before Mutation: {mutable_binding}");

    mutable_binding += 1;
    println!("After Mutation: {mutable_binding}");

    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
        println!("_mutable_integer = {_mutable_integer}"); // should equal 7
        
        //_mutable_integer = 50; // cannot mutate immutable variable (frozen)
    }

    _mutable_integer = 20;
    println!("_mutable_integer = {_mutable_integer}"); // should now equal 20
}
