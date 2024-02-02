use num_traits::Num;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn square_distance_from_origin(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 5 };
    println!("Distance = {}", p.square_distance_from_origin());
}
