fn main() {
    let n = 10;
    println!("{}", fibonacci_rec(n));
}

fn fibonacci_rec(n: u64) -> u64 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
    }
}
