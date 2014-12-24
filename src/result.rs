use libc;
use libnanomsg;

use std::str;
use std::fmt;
use std::io;
use std::io::{IoError, IoErrorKind};

pub use self::NanoErrorKind::*;

pub type NanoResult<T> = Result<T, NanoError>;

#[deriving(Show, Clone, PartialEq, FromPrimitive, Copy)]
pub enum NanoErrorKind {
    Unknown = 0i,
    OperationNotSupported = libnanomsg::ENOTSUP as int,
    ProtocolNotSupported = libnanomsg::EPROTONOSUPPORT as int,
    NoBufferSpace = libnanomsg::ENOBUFS as int,
    NetworkDown = libnanomsg::ENETDOWN as int,
    AddressInUse = libnanomsg::EADDRINUSE as int,
    AddressNotAvailable = libnanomsg::EADDRNOTAVAIL as int,
    ConnectionRefused = libnanomsg::ECONNREFUSED as int,
    OperationNowInProgress = libnanomsg::EINPROGRESS as int,
    NotSocket = libnanomsg::ENOTSOCK as int,
    AddressFamilyNotSupported = libnanomsg::EAFNOSUPPORT as int,
    WrongProtocol = libnanomsg::EPROTO as int,
    TryAgain = libnanomsg::EAGAIN as int,
    BadFileDescriptor = libnanomsg::EBADF as int,
    InvalidArgument = libnanomsg::EINVAL as int,
    TooManyOpenFiles = libnanomsg::EMFILE as int,
    BadAddress = libnanomsg::EFAULT as int,
    PermisionDenied = libnanomsg::EACCESS as int,
    NetworkReset = libnanomsg::ENETRESET as int,
    NetworkUnreachable = libnanomsg::ENETUNREACH as int,
    HostUnreachable = libnanomsg::EHOSTUNREACH as int,
    NotConnected = libnanomsg::ENOTCONN as int,
    MessageTooLong = libnanomsg::EMSGSIZE as int,
    Timeout = libnanomsg::ETIMEDOUT as int,
    ConnectionAbort = libnanomsg::ECONNABORTED as int,
    ConnectionReset = libnanomsg::ECONNRESET as int,
    ProtocolNotAvailable = libnanomsg::ENOPROTOOPT as int,
    AlreadyConnected = libnanomsg::EISCONN as int,
    SocketTypeNotSupported = libnanomsg::ESOCKTNOSUPPORT as int,
    Terminating = libnanomsg::ETERM as int,
    NameTooLong = libnanomsg::ENAMETOOLONG as int,
    NoDevice = libnanomsg::ENODEV as int,
    FileStateMismatch = libnanomsg::EFSM as int,
    Interrupted = libnanomsg::EINTR as int
}

#[deriving(PartialEq, Copy)]
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
            let c_desc = libnanomsg::nn_strerror(nn_errno);
            let desc = str::from_c_str(c_desc);

            NanoError::new(desc, error_kind)
        }
    }

    #[unstable]
    pub fn to_ioerror(&self) -> IoError {
        match self.kind {
            NanoErrorKind::Timeout => io::standard_error(IoErrorKind::TimedOut),
            NanoErrorKind::InvalidArgument => io::standard_error(IoErrorKind::InvalidInput),
            NanoErrorKind::BadFileDescriptor => io::standard_error(IoErrorKind::FileNotFound),
            NanoErrorKind::OperationNotSupported => io::standard_error(IoErrorKind::MismatchedFileTypeForOperation),
            NanoErrorKind::FileStateMismatch => io::standard_error(IoErrorKind::ResourceUnavailable),
            NanoErrorKind::Terminating => io::standard_error(IoErrorKind::IoUnavailable),
            NanoErrorKind::Interrupted => io::standard_error(IoErrorKind::BrokenPipe),
            _ => io::standard_error(io::OtherIoError)
        }
    }
}
/*
impl FromError<io::IoError> for NanoError {
    fn from_error(_: io::IoError) -> NanoError {
        NanoError::new("", TryAgain)
    }
}
*/
impl fmt::Show for NanoError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "An error has ocurred: Kind: {} Description: {}", self.kind, self.description)
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
    use std::io::{IoErrorKind};

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
}
