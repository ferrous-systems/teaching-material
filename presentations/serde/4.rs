extern crate serde_json;
extern crate toml;
extern crate serde_transcode;

use serde_transcode::transcode;

fn main() {
    let payload = "{\"id\":1,\"direction\":\"West\"}";
    let mut buffer = String::new();
    {
        let mut ser = toml::Serializer::new(&mut buffer);
        let mut de = serde_json::Deserializer::from_str(&payload);
        transcode(&mut de, &mut ser)
            .unwrap();
    }
    println!("{:?}", buffer);
}