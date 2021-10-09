// use std::io;

struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    let area = calc_area(&rect);
    println!("The area of a {} x {} rectangle is {}", rect.width, rect.height, area)
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
