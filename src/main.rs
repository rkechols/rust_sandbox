// use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width >= other.width && self.height >= other.height {
            true  // fits as-is
        } else if self.width >= other.height && self.height >= other.width {
            true  // fits if you swap axes
        } else {
            false  // doesn't fit
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(40);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!();
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
    println!("Can rect2 hold square? {}", rect2.can_hold(&square));
    println!("Can rect3 hold square? {}", rect3.can_hold(&square));
    println!();
    println!("The area of a {} x {} rectangle is {}", rect1.width, rect1.height, rect1.area());
    println!("This is the rectangle: {:?}", rect1)
}
