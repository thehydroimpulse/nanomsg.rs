use libc;
use libnanomsg;

use std::str;
use std::fmt;
use std::old_io;
use std::old_io::{IoError, IoErrorKind};
use std::error::FromError;
use std::num::FromPrimitive;
use std::ffi::c_str_to_bytes;
use std::mem::transmute;

pub use self::NanoErrorKind::*;

pub type NanoResult<T> = Result<T, NanoError>;

#[derive(Debug, Clone, PartialEq, FromPrimitive, Copy)]
pub enum NanoErrorKind {
    Unknown = 0is,
    OperationNotSupported = libnanomsg::ENOTSUP as isize,
    ProtocolNotSupported = libnanomsg::EPROTONOSUPPORT as isize,
    NoBufferSpace = libnanomsg::ENOBUFS as isize,
    NetworkDown = libnanomsg::ENETDOWN as isize,
    AddressInUse = libnanomsg::EADDRINUSE as isize,
    AddressNotAvailable = libnanomsg::EADDRNOTAVAIL as isize,
    ConnectionRefused = libnanomsg::ECONNREFUSED as isize,
    OperationNowInProgress = libnanomsg::EINPROGRESS as isize,
    NotSocket = libnanomsg::ENOTSOCK as isize,
    AddressFamilyNotSupported = libnanomsg::EAFNOSUPPORT as isize,
    WrongProtocol = libnanomsg::EPROTO as isize,
    TryAgain = libnanomsg::EAGAIN as isize,
    BadFileDescriptor = libnanomsg::EBADF as isize,
    InvalidArgument = libnanomsg::EINVAL as isize,
    TooManyOpenFiles = libnanomsg::EMFILE as isize,
    BadAddress = libnanomsg::EFAULT as isize,
    PermisionDenied = libnanomsg::EACCESS as isize,
    NetworkReset = libnanomsg::ENETRESET as isize,
    NetworkUnreachable = libnanomsg::ENETUNREACH as isize,
    HostUnreachable = libnanomsg::EHOSTUNREACH as isize,
    NotConnected = libnanomsg::ENOTCONN as isize,
    MessageTooLong = libnanomsg::EMSGSIZE as isize,
    Timeout = libnanomsg::ETIMEDOUT as isize,
    ConnectionAbort = libnanomsg::ECONNABORTED as isize,
    ConnectionReset = libnanomsg::ECONNRESET as isize,
    ProtocolNotAvailable = libnanomsg::ENOPROTOOPT as isize,
    AlreadyConnected = libnanomsg::EISCONN as isize,
    SocketTypeNotSupported = libnanomsg::ESOCKTNOSUPPORT as isize,
    Terminating = libnanomsg::ETERM as isize,
    NameTooLong = libnanomsg::ENAMETOOLONG as isize,
    NoDevice = libnanomsg::ENODEV as isize,
    FileStateMismatch = libnanomsg::EFSM as isize,
    Interrupted = libnanomsg::EINTR as isize
}

#[derive(PartialEq, Copy)]
pub struct NanoError {
    pub description: &'static str,
    pub kind: NanoErrorKind
}

impl NanoError {
    #[unstable]
    pub fn new(description: &'static str, kind: NanoErrorKind) -> NanoError {
        NanoError {
            description: description,
            kind: kind
        }
    }

    #[unstable]
    pub fn from_nn_errno(nn_errno: libc::c_int) -> NanoError {
        let maybe_error_kind = FromPrimitive::from_i64(nn_errno as i64);
        let error_kind = maybe_error_kind.unwrap_or(Unknown);

        unsafe {
            let c_desc: *const libc::c_char = libnanomsg::nn_strerror(nn_errno);
            let c_desc_ref: &'static *const libc::c_char = transmute(&c_desc);
            let desc_bytes = c_str_to_bytes(c_desc_ref);
            let desc = str::from_utf8(desc_bytes).unwrap_or("Error");

            NanoError::new(desc, error_kind)
        }
    }

    #[unstable]
    pub fn to_ioerror(&self) -> IoError {
        match self.kind {
            NanoErrorKind::Timeout => old_io::standard_error(IoErrorKind::TimedOut),
            NanoErrorKind::InvalidArgument => old_io::standard_error(IoErrorKind::InvalidInput),
            NanoErrorKind::BadFileDescriptor => old_io::standard_error(IoErrorKind::FileNotFound),
            NanoErrorKind::OperationNotSupported => old_io::standard_error(IoErrorKind::MismatchedFileTypeForOperation),
            NanoErrorKind::FileStateMismatch => old_io::standard_error(IoErrorKind::ResourceUnavailable),
            NanoErrorKind::Terminating => old_io::standard_error(IoErrorKind::IoUnavailable),
            NanoErrorKind::Interrupted => old_io::standard_error(IoErrorKind::BrokenPipe),
            _ => {
                IoError {
                    kind: IoErrorKind::OtherIoError,
                    desc: self.description,
                    detail: None
                }
            }
        }
    }
}

impl FromError<old_io::IoError> for NanoError {
    fn from_error(io_err: old_io::IoError) -> NanoError {
        match io_err.kind {
            IoErrorKind::TimedOut => NanoError::new(io_err.desc, NanoErrorKind::Timeout),
            IoErrorKind::InvalidInput => NanoError::new(io_err.desc, NanoErrorKind::InvalidArgument),
            IoErrorKind::FileNotFound => NanoError::new(io_err.desc, NanoErrorKind::BadFileDescriptor),
            IoErrorKind::MismatchedFileTypeForOperation => NanoError::new(io_err.desc, NanoErrorKind::OperationNotSupported),
            IoErrorKind::ResourceUnavailable => NanoError::new(io_err.desc, NanoErrorKind::FileStateMismatch),
            IoErrorKind::IoUnavailable => NanoError::new(io_err.desc, NanoErrorKind::Terminating),
            IoErrorKind::BrokenPipe => NanoError::new(io_err.desc, NanoErrorKind::Interrupted),
            _ => NanoError::new(io_err.desc, Unknown)
        }
    }
}

impl fmt::Debug for NanoError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "An error has ocurred: {}", self.description)
    }
}

impl fmt::Display for NanoError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "An error has ocurred: {}", self.description)
    }
}
pub fn last_nano_error() -> NanoError {
    let nn_errno = unsafe { libnanomsg::nn_errno() };

    NanoError::from_nn_errno(nn_errno)
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use libnanomsg;
    use libc;
    use super::NanoErrorKind::*;
    use super::NanoErrorKind;
    use super::NanoError;
    use std::old_io;
    use std::old_io::{IoErrorKind};
    use std::error::FromError;
    use std::num::FromPrimitive;

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
        assert_convert_error_code_to_error_kind(libnanomsg::ENOTSUP, OperationNotSupported);
        assert_convert_error_code_to_error_kind(libnanomsg::EPROTONOSUPPORT, ProtocolNotSupported);
        assert_convert_error_code_to_error_kind(libnanomsg::EADDRINUSE, AddressInUse);
        assert_convert_error_code_to_error_kind(libnanomsg::EHOSTUNREACH, HostUnreachable);
    }

    fn check_error_kind_match(nano_err_kind: NanoErrorKind, io_err_kind: IoErrorKind) {
        let nano_err = NanoError::from_nn_errno(nano_err_kind as libc::c_int);
        let io_err = nano_err.to_ioerror();

        assert_eq!(io_err_kind, io_err.kind)
    }

    #[test]
    fn check_to_ioerror() {
        check_error_kind_match(NanoErrorKind::Timeout, IoErrorKind::TimedOut);
        check_error_kind_match(NanoErrorKind::InvalidArgument, IoErrorKind::InvalidInput);
        check_error_kind_match(NanoErrorKind::BadFileDescriptor, IoErrorKind::FileNotFound);
        check_error_kind_match(NanoErrorKind::OperationNotSupported, IoErrorKind::MismatchedFileTypeForOperation);
        check_error_kind_match(NanoErrorKind::FileStateMismatch, IoErrorKind::ResourceUnavailable);
        check_error_kind_match(NanoErrorKind::Terminating, IoErrorKind::IoUnavailable);
        check_error_kind_match(NanoErrorKind::Interrupted, IoErrorKind::BrokenPipe);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = old_io::standard_error(IoErrorKind::TimedOut);
        let nano_err: NanoError = FromError::from_error(io_err);

        assert_eq!(NanoErrorKind::Timeout, nano_err.kind)
    }
}
