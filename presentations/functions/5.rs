fn prints_anything<T: Debug>(thing: T) {
    println!("{:?}", thing);
}

fn prints_anything<T>(thing: T)
where
    T: Debug,
{
    println!("{:?}", thing);
}
