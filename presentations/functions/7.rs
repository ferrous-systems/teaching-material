struct Square(f32);

impl Square {
    fn num_sides() -> u32 {
        4
    }
}

fn main() {
    println!("Num sides: {}", Square::num_sides());
}
