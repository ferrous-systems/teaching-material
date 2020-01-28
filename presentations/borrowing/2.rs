fn main() {
    let mut point = Point { x: 1, y: 2 };
    let re = &point; // -\ <1>
    point.x = 2;     //  | <2>
    inspect(re);     // -/ <3>
}
