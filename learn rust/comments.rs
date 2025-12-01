// Regular comments in Rust start with two slashes
// They are used to explain code and make it more understandable
// This file is intended to demonstrate the use of comments in Rust

fn main() {
    // This is a comment
    // println!("Hello from comments.rs!"); will be ignored by the compiler
    println!("Trying  to hightlight comments!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}