fn main() {
    let x = increment(5);

    println!("The value of x is: {x}");
}

fn increment(x: i32) -> i32 {
    return x + 1;
}
