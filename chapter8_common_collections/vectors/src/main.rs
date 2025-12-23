fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);

    println!("Vector1 With Type Annotation: {v1:?}");
    println!("Vector2 With Macro Definition: {v2:?}");

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is: {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element using get is: {third}"),
        None => println!("There is no third element"),
    }

    for i in &v3 {
        println!("{i}");
    }

    let mut v4 = vec![12, 34, 87, 109, 172];
    
    for i in &mut v4 {
        *i += 50;
    }

    println!("New v4 values: {v4:?}");

    {
        let _v5 = vec![10, 20, 30];
        let second = _v5[1];
        println!("The second element in _v5 is: {second}");
        println!("{_v5:?}")
    } // v5 and all its values are dropped when the scope ends

    let v6 = vec![String::from("first"), String::from("second"), String::from("third")];
    let _last = &v6[2];
}
