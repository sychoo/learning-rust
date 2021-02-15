// Fri Feb 12 14:42:02 EST 2021
// Fibonacci Sequence fib(3) = fib(2) + fib(10)

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    println!("{}", fib(2));
    println!("{}", fib(10)); // 55

}

