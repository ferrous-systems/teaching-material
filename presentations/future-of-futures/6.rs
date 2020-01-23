async fn get_and_parse_some_data() -> Result<Data, NetworkError> {
    // ...
}

fn main() -> Result<(), Box<Error>> {
    // ... init run loop

    let data: MyData = await!(get_and_parse_some_data()?);
}