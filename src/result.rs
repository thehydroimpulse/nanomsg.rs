use std::str::SendStr;

pub type NanoResult<T> = Result<T, NanoError>;

#[deriving(Show, Clone, PartialEq)]
pub enum ErrorKind {
    Unknown,
    SocketInitializationError
}

#[deriving(Show, PartialEq)]
pub struct NanoError {
    description: SendStr,
    kind: ErrorKind
}

impl<T: IntoMaybeOwned<'static>> NanoError {
    pub fn new(description: T, kind: ErrorKind) -> NanoError {
        NanoError {
            description: description.into_maybe_owned(),
            kind: kind
        }
    }

    pub fn to_str(&self) -> String {
        format!("An error has ocurred: Kind: {} Description: {}", self.kind, self.description)
    }
}
