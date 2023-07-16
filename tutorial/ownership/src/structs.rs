use std::fmt::{Debug};

pub fn exported_fn() {
    println!("Hello from module!");
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub center: Point,
    width: u32,
    height: u32,
}

impl Point {
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
}

impl Rectangle {
    pub fn square(x: i32, y: i32, size: u32) -> Self {
        return Self::new(x, y, size, size);
    }

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        return Self {
            center: Point { x, y },
            width,
            height,
        };
    }
    pub fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

// pub fn build_rectangle(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
//     return Rectangle {
//         center: Point { x, y },
//         width,
//         height,
//     };
// }
//
// pub fn rectangle_area(rect: &Rectangle) -> u32 {
//     return rect.width * rect.height;
// }
