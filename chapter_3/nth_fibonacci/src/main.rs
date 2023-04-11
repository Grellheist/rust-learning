fn main() {
    const N: i32 = 10;
    println!("The {}th Fibonacci number is {}", N, fib(N));
}

fn fib(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
