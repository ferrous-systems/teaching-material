fn main() {
    let words = "Cow says moo";
    let each: Vec<_> = words.split(" ").collect();
    println!("{:?}", each);
}