extern crate serde_json;

use serde_json::Value;

fn main() {
    let data = r#" { "name": "John Doe", "age": 43, ... } "#;
    let v: Value = serde_json::from_str(data)?;
    println!(
        "Please call {} at the number {}", 
        v["name"], 
        v["phones"][0]
    );
}
