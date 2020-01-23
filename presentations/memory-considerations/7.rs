enum Test {
    Foo,
    Bar
}

fn main() {
    println!("Bare Values");
    println!("{:?}", std::mem::size_of::<String>());
    println!("{:?}", std::mem::size_of::<i32>());
    println!("{:?}", std::mem::size_of::<Test>());

    println!("Results:");
    println!("{:?}", std::mem::size_of::<Result<i32,String>>());
    println!("{:?}", std::mem::size_of::<Result<i32,&String>>());
    println!("{:?}", std::mem::size_of::<Result<i32,Test>>());

    println!("Options:");
    println!("{:?}", std::mem::size_of::<Option<i32>>());
    println!("{:?}", std::mem::size_of::<Option<Test>>());
}