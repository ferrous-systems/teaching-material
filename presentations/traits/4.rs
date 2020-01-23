struct Point {
    x: i32,
    y: i32
}

trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

impl Distance for Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", p1.distance(&p2));
}