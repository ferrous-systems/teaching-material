use std::{
    borrow::{Borrow, Cow},
    fmt::Display,
    str::FromStr,
};

use semver::Error as RepositoryError;
use semver::{Crate, SemVer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Update {
    pub crate_name: String,
    pub version: SemVer,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResult(pub Result<Option<String>, ApiError>);

impl TryFrom<ApiResult> for String {
    type Error = serde_json::Error;

    fn try_from(value: ApiResult) -> Result<Self, Self::Error> {
        serde_json::to_string(&value)
    }
}

impl TryFrom<&str> for ApiResult {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    Get(String),
    Put(Crate),
    Update(Update),
}

impl FromStr for Command {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_without_newline = s.split_once("\n");

        command_without_newline
            .ok_or_else(|| ApiError::ParseError("missing newline".to_string()))
            .and_then(|(s, _)| {
                serde_json::from_str(s).map_err(|serde_error| {
                    ApiError::ParseError(format!("serde: {:?}", serde_error))
                })
            })
    }
}

impl TryFrom<Command> for String {
    type Error = serde_json::Error;

    fn try_from(value: Command) -> Result<Self, Self::Error> {
        serde_json::to_string(&value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    InvalidCommand,
    InvalidData(RepositoryError),
    // usually you'd wrap the underlying error and not serialize to string -
    // we do it here to always be Serializable
    ParseError(String),
    Internal,
}

impl From<RepositoryError> for ApiError {
    fn from(e: RepositoryError) -> Self {
        Self::InvalidData(e)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ApiError::InvalidCommand => Cow::from("invalid command"),
            ApiError::ParseError(_) => Cow::from("invalid payload"),
            ApiError::Internal => Cow::from("internal"),
            ApiError::InvalidData(e) => Cow::from(format!("underlying: {:?}", e)),
        };

        f.write_str(s.borrow())
    }
}

impl std::error::Error for ApiError {}
