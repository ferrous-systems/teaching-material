fn main() {
    let vec = vec![1, 2, 3];
    let iter = vec.iter(); // <1>
    drop(vec); // <2>
    for i in iter { // <3>
        println!("{}", i);
    }
}
