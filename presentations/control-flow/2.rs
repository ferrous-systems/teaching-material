fn main() {
    let a = 4;
    match a % 3 {
        0 => { println!("divisible by 3") }, // <1>
        _ => { println!("not divisible by 3") }, // <2>
    }
}
