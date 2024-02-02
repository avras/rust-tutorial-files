fn main() {
    let n = 10;
    println!("{}", fibonacci_rec(n));
}

fn fibonacci_rec(n: u64) -> u64 {
    match n {
        1 | 2 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
