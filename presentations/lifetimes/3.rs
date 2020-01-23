struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn x(&self) -> &i32 {
        &self.x
    }

    fn y<'point>(&'point self) -> &'point i32 {
        &self.y
    }
}
