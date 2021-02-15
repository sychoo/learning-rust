// Fri Feb 12 14:28:01 EST 2021
// test ownership

fn main() {
    let s1 = String::from("This is a string");
    let s2 = s1;
    println!("{}", s1); // string is moved from s1 to s2
}
