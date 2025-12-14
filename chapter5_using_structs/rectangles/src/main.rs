#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated function (in this case a constructor) to create a new instance of Rectangle
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn change_height(&mut self, height: u32) {
        self.height = height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::new(20, 30);
    dbg!(&rect3);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("rect1 is {rect1:#?}");
    
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    rect1.change_height(40);
    dbg!(&rect1);

    if rect1.width() {
        println!("The rectangle has a non zero width, it is: {}", rect1.width);
    }
}

// we use a borrow to not take ownership of rect1
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
