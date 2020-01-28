struct ExampleIter<'iter, T> { <2>
    vec: &'iter Vec<T>, <1>
    pos: usize,
}

fn main() {
    let vec: Vec<u32> = vec![1,2,3]; <4>
    let iter: Iter<'_, u32> = vec.iter(); <3> <4>
    for i in iter {
        println!("{}", i);
    }
}
