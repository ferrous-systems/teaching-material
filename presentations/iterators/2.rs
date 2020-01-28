fn main() {
    let vec = vec![1,2,3];
    let iter = vec.iter();

    for i in iter {
        println!("{}", i);
    }

    println!("{:?}", vec);
}
