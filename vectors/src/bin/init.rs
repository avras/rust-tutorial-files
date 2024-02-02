fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(10);
    v.push(11);
    println!("{:?}", v);

    v = vec![4, 5, 6];
    println!("{:?}", v);

    v = vec![10; 5];
    println!("{:?}", v);
}
