fn main() {
    let x: (u8, f32, bool) = (38, 5.5, true);

    // Note the {:?}
    println!("{:?}", x);
    println!("{} {} {}", x.0, x.1, x.2);
}
