fn main() {
    let mut p = Point { x: 1, y: 2 };
    inspect(&p); <4>
    move_point(&mut p,3,3); <3>
    inspect(&p); <4>
}

fn move_point(
    p: &mut Point, <1> <2>
    x: i32, y: i32
) {
    p.x = x;
    p.y = y;
}
