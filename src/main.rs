// use std::io;

struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    let area = calc_area(&rect);
    println!("The area of a {} x {} rectangle is {}", rect.width, rect.height, area)
}

fn calc_area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
