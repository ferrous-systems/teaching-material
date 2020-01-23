fn main() {
    let vec = vec![1,2,3];
    let double = |x| { x * 2 };
    let out = vec.iter().map(double).collect::<Vec<_>>();
    println!("{:?}", out);
}