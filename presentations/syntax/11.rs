fn main() {
    let foo = 1;
    let bar = {
        // Shadows earlier declaration.
        let foo = 2;
        foo
    };
    println!("{}", foo);
    println!("{}", bar);
}