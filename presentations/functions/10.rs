struct Square(f32);

impl Square {
    fn calc_area(&self) -> f32 {
        self.0 * self.0
    }

    fn destroy(self) {
        println!("I ate the square. It has gone.");
    }
}

fn main() {
    let sq = Square(1.0);
    println!("Area: {}", sq.calc_area());
    sq.destroy();
    // This line won't compile
    // println!("Area: {}", sq.calc_area());
}
