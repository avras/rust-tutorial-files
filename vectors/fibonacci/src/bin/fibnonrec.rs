fn main() {
    let n = 10;
    println!("{}", fibonacci_nonrec(n));
}

fn fibonacci_nonrec(n: u64) -> u64 {
    match n {
        1 | 2 => 1,
        _ => {
            let mut curr = 1;
            let mut prev = 1;
            let mut sum = 0;
            for _i in 2..n {
                sum = curr + prev;
                prev = curr;
                curr = sum;
            }
            sum
        }
    }
}
