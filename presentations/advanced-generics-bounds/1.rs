fn print_everything<T: Debug>(to_print: T) {
    println!("{:?}", to_print);
}

fn print_everything2<T>(to_print: T)
    where T: Debug
{
    println!("{:?}", to_print);
}