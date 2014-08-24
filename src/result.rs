use std::str::SendStr;

pub type NanoResult<T> = Result<T, NanoError>;

#[deriving(Show, Clone, PartialEq)]
pub enum ErrorKind {
    Unknown
}

#[deriving(Show, PartialEq)]
pub struct NanoError {
    description: SendStr,
    kind: ErrorKind
}

impl NanoError {
    pub fn new(kind: ErrorKind, description: SendStr) -> NanoError {
        NanoError {
            description: description,
            kind: kind
        }
    }

    pub fn to_str(&self) -> String {
        format!("An error has ocurred: Kind: {} Description: {}", self.kind, self.description)
    }
}
