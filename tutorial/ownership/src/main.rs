mod structs;
mod enums;

use enums::Message;
use structs::Rectangle;

use std::io;
use std::io::Write;

fn main() {
    Message::Join(String::from("a")).log();
    Message::Leave(String::from("a")).log();
    Message::Send(String::from("a"), String::from("bcde")).log();

    structs::exported_fn();
    let r = Rectangle::new(0, 0, 2, 8);
    let s = Rectangle::square(1, 1, 3);
    // println!("Built rectangle: {:?}, area = {}", r, structs::rectangle_area(&r));
    println!("Built rectangle: {:?}, area = {}", r, r.area());
    s.center.x();
    s.center.y();

    let input = prompt("Enter a string: ");

    println!("Input is {}", input);
    let len = calculate_length(&input);

    println!("The length of '{}' is {}.", input, len);

    let mut changed_input = input;
    change(&mut changed_input);
}

fn prompt(prompt_text: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt_text);
    io::stdout().flush().expect("Could not flush stdout");
    io::stdin().read_line(&mut input).expect("Could not read line");

    input.trim().to_string()
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
