fn main() {
    let slice: &[i32] = &[1,2,3,4];
    let subslice = &slice[1..2];
    println!("subslice = {:?}", subslice);
}
