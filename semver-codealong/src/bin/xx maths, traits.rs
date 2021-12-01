// trait VeryFormattable: Debug + Display {}
// impl<T> VeryFormattable for T where T: Debug + Display {}

// struct A {}
// struct B {}
trait VeryFormattable: Debug + Display {
    fn show(&self) {
        println!("{} ... or {:?}", self, self);
    }
}

use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

struct Point<G> {
    x: i32,
    y: i32,
    geometry: PhantomData<G>,
}

struct Euclidean {}
struct Hyperbolic {}

trait Distance<OtherShape> {
    fn distance(&self, other: &OtherShape) -> f64;
}

impl Distance<Point<Euclidean>> for Point<Euclidean> {
    fn distance(&self, other: &Self) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

impl Distance<Point<Hyperbolic>> for Point<Hyperbolic> {
    fn distance(&self, other: &Self) -> f64 {
        let x1 = self.x as f64;
        let x2 = other.x as f64;

        let y1 = self.y as f64;
        let y2 = other.y as f64;

        (y1.cosh() * (x2 - x1).cosh() * y2.cosh() - y1.sinh() * y2.sinh()).acosh()
    }
}

enum Either<L, R> {
    Left(L),
    Right(R),
}
fn bla() {
    let e: Either<_, f64> = Either::Left(1);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
