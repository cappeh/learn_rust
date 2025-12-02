fn main() {
    let _num: u32 = "96".parse().expect("not a number!");
    let _num2 = "42".parse::<i32>().unwrap();
    println!("_num2 = {_num2}");
}
