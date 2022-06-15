struct Square(f32);

impl Square {
    fn calc_area(&self) -> f32 {
        self.0 * self.0
    }

    fn double_size(&mut self) {
        self.0 *= 2.0;
    }
}

fn main() {
    let mut sq = Square(1.0);
    println!("Area: {}", sq.calc_area());
    sq.double_size();
    println!("New Area: {}", sq.calc_area());
}
