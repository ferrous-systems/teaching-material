fn main() {
    // &'static str
    let this = "Hallo";
    // String
    let that: String = String::from("Hallo");
    // &str
    let other = that.as_str();
}