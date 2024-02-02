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

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

fn main() {
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Rectangle has area {}", r.area());

    let c = Circle { radius: 1.0 };
    println!("Circle has area {}", c.area());
}
