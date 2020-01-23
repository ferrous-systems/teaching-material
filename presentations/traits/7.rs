#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

trait MyAddition<Other> {
    type Output;

    fn add(&self, other: &Other) -> Self::Output;
}

impl MyAddition<Point> for Point {
    type Output = Point;

    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    println!("{:?}", p1.add(&p2))
}