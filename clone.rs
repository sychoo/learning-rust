// Fri Feb 12 15:29:37 EST 2021
// test clone function

fn main() {
    let x = String::from("Hello, World!");
    let mut y = x.clone(); // copies the heap data
    y.push_str("hello");
    println!("x = {}, y = {}", x, y);
}
