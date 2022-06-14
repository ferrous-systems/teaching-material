// Get the inner type from Option
let item = returns_option();
if let Some(item) = item { 
    println!("{:?}", item);
}

// Use shadowing to make the variable immutable outside of 
// where it needs to be mutable
let mut data = 42;
// change the data 
data += 1;
// Shadow using `let` again
let data = data; 
// data is immutable from now on