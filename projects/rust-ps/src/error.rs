use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("cannot read status file. detail: {0}")]
    CannotReadStatusFile(String),
    #[error("cannot parse. detail: {0}")]
    ParseError(String),
    #[error("cannot deserialize process status file. detail: {0}")]
    CannotDeserializeStatusFile(String),
}

impl From<std::string::String> for Error {
    fn from(value: std::string::String) -> Self {
        Self::Message(value.into())
    }
}
