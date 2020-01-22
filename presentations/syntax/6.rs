fn main() {    
    // Loop over iterator
    let range = 0..10;
    for i in range {
        // ...
    }
    // while let
    let mut range = 0..10;
    while let Some(v) = range.next() {
        // ...
    }
}