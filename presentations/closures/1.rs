fn main() {
    let vec = vec![1,2,3];
    let out = vec.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}", out);
}