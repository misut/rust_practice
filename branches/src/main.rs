fn fib(n: u128) -> u128 {
    if n <= 1 {
        1
    }
    else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    for n in (1..11) {
        println!("{}-th Fibonacci number: {}", n, fib(n));
    }
}
