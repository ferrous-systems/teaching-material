#[derive(Debug)]
struct Point {
    x: i32,
    y: i32 
}

fn main() {
    let mut p = Point { x: 1, y: 2 }; <1>
    inspect(&p); <3>
    p.x = 2; <4>
    inspect(&p);
    <5>
}

fn inspect(p: &Point) { <2>
    println!("{:?}", p);
}
