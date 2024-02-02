fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    let s: u32 = v_iter.sum();
    println!("Sum: {}", s);
}
