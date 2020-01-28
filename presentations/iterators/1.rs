fn main() {
    let vec = vec![1,2,3];
    let iter = vec.into_iter();

    for i in iter {
        println!("{}", i);
    }

    //println!("{:?}", vec); <1>
}
