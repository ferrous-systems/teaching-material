struct Square(f32);

fn square_num_sides() -> u32 {
    4
}

fn square_calc_area(square: &Square) -> f32 {
    square.0 * square.0
}

fn square_double_size(square: &mut Square) {
    square.0 *= 2.0;
}
