use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
    TrailingData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing is command: {:?}!", self)
    }
}

impl std::error::Error for Error {}

pub fn parse(input: &str) -> Result<Command, Error> {
    if let Some(pos) = input.find('\n') {
        if !((pos + 1) == input.len()) {
            return Err(Error::TrailingData);
        }
    } else {
        return Err(Error::IncompleteMessage);
    }

    let mut split = input.splitn(2, ' ');

    if let Some(verb) = split.next() {
        match verb.trim() {
            "RETRIEVE" => {
                if split.next().is_none() {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = split.next() {
                    Ok(Command::Publish(String::from(payload.trim())))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            "" => Err(Error::EmptyMessage),
            _ => Err(Error::UnknownVerb),
        }
    } else {
        Err(Error::EmptyMessage)
    }
}
