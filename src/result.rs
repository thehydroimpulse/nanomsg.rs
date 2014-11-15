extern crate libnanomsg;

use std::str::SendStr;

pub type NanoResult<T> = Result<T, NanoError>;

#[deriving(Show, Clone, PartialEq)]
pub enum ErrorKind {
    Unknown,
    SocketInitializationError,
    SocketBindError,
    SocketBufferError,
    SocketOptionError,
    ShutdownError
}

#[deriving(Show, Clone, PartialEq, FromPrimitive)]
pub enum NanoErrorKind {
    NotSupported = libnanomsg::ENOTSUP as int,
    ProtocolNotSupported = libnanomsg::EPROTONOSUPPORT as int
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

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    #[phase(plugin, link)]
    extern crate log;
    extern crate libnanomsg;
    extern crate libc;

    use super::*;

    fn assert_convert_error_code_to_error_kind(error_code: libc::c_int, expected_error_kind: NanoErrorKind) {
        let i64_error_code = error_code as i64;
        let converted_error_kind = FromPrimitive::from_i64(i64_error_code);

        match converted_error_kind {
            Some(error_kind) => assert_eq!(expected_error_kind, error_kind),
            None => panic!("Failed to convert error code to NanoErrorKind")
        }
    }

    #[test]
    fn can_convert_error_code_to_error_kind() {
        assert_convert_error_code_to_error_kind(libnanomsg::ENOTSUP, NotSupported);
        assert_convert_error_code_to_error_kind(libnanomsg::EPROTONOSUPPORT, ProtocolNotSupported);
    }

}
