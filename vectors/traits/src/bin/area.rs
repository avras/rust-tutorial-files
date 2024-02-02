pub trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Rectangle has area {}", r.area());
}
