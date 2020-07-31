extern crate redisish;

use redisish::*;

#[test]
fn test_retrieve() {
    let line = "RETRIEVE\n";
    let result = redisish::parse(line);
    assert_eq!(result, Ok(Command::Retrieve));
}

#[test]
fn test_publish() {
    let line = "PUBLISH TestMessage\n";
    let result = redisish::parse(line);
    assert_eq!(result, Ok(Command::Publish("TestMessage".into())));
}

#[test]
fn test_empty_string() {
    let line = "";
    let result = redisish::parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_empty_message() {
    let line = "\n";
    let result = redisish::parse(line);
    assert_eq!(result, Err(Error::EmptyMessage));
}

#[test]
fn test_missing_newline() {
    let line = "FooBar";
    let result = redisish::parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_retrieve_with_payload() {
    let line = "RETRIEVE Payload\n";
    let result = redisish::parse(line);
    assert_eq!(result, Err(Error::UnexpectedPayload));
}

#[test]
fn test_publish_without_payload() {
    let line = "PUBLISH\n";
    let result = redisish::parse(line);
    assert_eq!(result, Err(Error::MissingPayload));
}

#[test]
fn test_publish_with_empty_payload() {
    let line = "PUBLISH \n";
    let result = redisish::parse(line);
    assert_eq!(result, Ok(Command::Publish("".into())));
}

#[test]
fn test_inline_newline() {
    let line = "PUBLISH fooo\nbar\n";
    let result = redisish::parse(line);
    assert_eq!(result, Ok(Command::Publish("fooo".into())));    
}