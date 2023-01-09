/// Represents a square, a shape with four equal sides.
pub struct Square {
    /// The length of one side of the square
    side: f32
}

impl Square {
    /// Construct a new [`Square`] with the given length for each side.
    pub  fn new(side: f32) -> Square {
        Square {
            side
        }
    }

    /// Calculate the area of the given [`Square`]
    pub  fn area(self: &Square) -> f32 {
        self.side * self.side
    }

    /// Calculate the perimeter of the given [`Square`]
    pub  fn perimeter(self: &Square) -> f32 {
        self.side * 4.0
    }
}

/// Represents a circle, with a given radius.
pub struct Circle {
    radius: f32
}

impl Circle {
    /// Construct a new [`Circle`] with the given radius.
    pub  fn new(radius: f32) -> Circle {
        Circle {
            radius
        }
    }

    /// Calculate the area of the given [`Circle`]
    pub  fn area(self: &Circle) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    /// Calculate the perimeter of the given [`Circle`]
    pub  fn perimeter(self: &Circle) -> f32 {
        std::f32::consts::PI * self.radius * 2.0
    }
}

/// Represents a Right Angled Triangle.
/// 
/// Specified with the length of the two sides adjacent to the right-angle.
pub struct RightAngleTriangle {
    along: f32,
    up: f32
}

impl RightAngleTriangle {
    /// Construct a new [`RightAngleTriangle`] with the given radius.
    pub fn new(along: f32, up: f32) -> RightAngleTriangle {
        RightAngleTriangle {
            along, up
        }
    }

    /// Calculate the area of the given [`RightAngleTriangle`]
    pub fn area(self: &RightAngleTriangle) -> f32 {
        self.along * self.up / 2.0
    }

    /// Calculate the perimeter of the given [`RightAngleTriangle`]
    pub fn perimeter(self: &RightAngleTriangle) -> f32 {
        let hypotenuse = (self.along * self.along + self.up * self.up).sqrt();
        self.along + self.up + hypotenuse
    }
}

pub enum Shape {
    Square(Square),
    Circle(Circle),
    RightAngleTriangle(RightAngleTriangle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let test_square = Square::new(5.0);
        assert_eq!(test_square.area(), 25.0);
        assert_eq!(test_square.perimeter(), 20.0);
    }

    #[test]
    fn rightangletriangles() {
        let test_triangle = RightAngleTriangle::new(3.0, 4.0);
        assert_eq!(test_triangle.area(), 6.0);
        assert_eq!(test_triangle.perimeter(), 12.0);
    }
}
