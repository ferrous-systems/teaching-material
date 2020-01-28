fn main() {
    let mut vec = vec![1,2,3];
    let iter_mut = vec.iter_mut();
    
    for i in iter_mut {
       *i += 1
    }

    println!("{:?}", vec);
}
