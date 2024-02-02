fn main() {
    let v1 = vec![15, 23, 44, 19, 56, 83, 33, 19, 76, 10];
    let v2 = vec![76, 23, 27, 20, 56, 83, 39, 19, 92, 60];

    let matches: u32 = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| if a == b { 1 } else { 0 })
        .sum();
    println!("Num matches = {matches}");
}
