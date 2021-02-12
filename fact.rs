// Thu Feb 11 11:46:52 EST 2021
// factorial

fn f1(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        n * f1(n - 1)
    }
}

fn f2(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * f2(n - 1),
    }
}

fn main() {
    let x = 10;
    println!("{}", f1(x));
    println!("{}", f2(x));
}
