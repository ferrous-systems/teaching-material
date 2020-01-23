fn main() {
    use serde_json::{self, Value};

    let data = r#" { "name": "John Doe", "age": 43, ... } "#;
    let v: Value = serde_json::from_str(data)?;
    println!(
        "Please call {} at the number {}", 
        v["name"], 
        v["phones"][0]
    );
}
