struct Point {
    x: i32,
    y: i32
}

trait Distance<OtherShape> {
    fn distance(&self, other: &OtherShape) -> f64;
}

impl Distance<Point> for Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    println!("{}", <Point as Distance<Point>>::distance(&p1, &p2));
}
