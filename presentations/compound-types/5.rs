struct Point(i32,i32);

fn main() {
    let p = Point(1, 2);
    println!("{}", p.0);
    println!("{}", p.1);
}