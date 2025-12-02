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

}
