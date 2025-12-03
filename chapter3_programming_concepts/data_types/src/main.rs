use std::io;

fn main() {
    let _num: u32 = "96".parse().expect("not a number!");
    let _num2 = "42".parse::<i32>().unwrap(); // turbofish syntax
    println!("_num2 = {_num2}");

    let _decimal = 98_222; // 98200
    let _hex = 0xFF; // 255 in decimal
    let _octal = 0o77; // 63 in decimal
    let _binary = 0b1111_0000; // 240 in decimal
    let _byte = b'A'; // 65 in decimal
    let _number = 96u32; // 96 in decimal (type u32)
    
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    let _t = true;
    let _f: bool = false;

    let _heart_eyed_cat = '😻';
    let _c: char = 'c';

    let tup: (i32, f64, u8) = (500, 6.4, 1); // can also be written as "let tup = (500, 6.4, 1)"
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}"); // prints 6.4

    let _five_hundred = tup.0; // access the first element in the tuple: tup

    // () 
    // this is the unit tuple. represents an empty value or empty return type
    // expressions implicitly return the unit value if they dont return any other value
    // an expression evaluates to a value such as x + y

    let _months = ["January", "February", "March", "April", "May", "June", "July",
                   "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // writes the type of each element and the number of
                                        // elements in the array type = i32, 5 elements in the
                                        // array

    let _b = [3; 5]; // = [3, 3, 3, 3, 3]

    let _april = _months[3];
    let _december = _months[_months.len() - 1];

    {
        let c: [i32; 5] = [2, 4, 6, 8, 10];
        println!("Choose an array index");

        let mut index = String::new();

        io::stdin().read_line(&mut index).expect("failed to read line");

        // a usize is required here because it guaranteed to be large enough to represent any
        // memory address on the system
        // if you used a u32 for example on a large array it may not be able to represent the
        // memory address
        let index: usize = index.trim().parse().expect("Not a number!");

        let val = c[index];

        println!("The value at index: {index} is: {val}");
    }
}
