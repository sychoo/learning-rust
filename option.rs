// Thu Feb 11 11:46:30 EST 2021
// greet and div

fn g(n: Option<String>) {
    match n {
        Some(n) => println!("Hello there, {}", n),
        None => println!("Howdy, Stranger"),
    }
}

// divide
fn d(n1: i32, n2: i32) -> Option<i32> {
    if n2 != 0 {
        Some(n1 / n2)
    } else {
        None
    }
}

// print divide result
fn pd(n1: i32, n2: i32) {
    match d(n1, n2) {
        Some(q) => println!("{} / {} = {}", n1, n2, q), // q = quotient = d(n1, n2)
        None => println!("{} / {} Failed", n1, n2),
    }
}

fn main() {
    g(Some("Simon".to_string()));
    g(None);

    pd(6, 3);
    pd(2, 0);
}

