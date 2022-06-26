fn main() {
    // &'static str
    let this = "Hello";
    // String
    let that: String = String::from("Hello");
    // &str
    let other = that.as_str();
}
