// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn x_and_y(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[allow(dead_code)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("the largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("the largest char is {result}");

    let p1 = Point { x: 5, y: 10 };
    println!("p1 x value is: {}", p1.x());

    let p3 = Point { x: 3.2, y: 4.2 };
    let (left, right) = p3.x_and_y();
    println!("{left}, {right}")
}
