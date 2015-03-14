use libc;
use libnanomsg;

use std::str;
use std::fmt;
use std::io;
use std::error::FromError;
use std::num::FromPrimitive;
use std::ffi::CStr;

pub use self::NanoErrorKind::*;

pub type NanoResult<T> = Result<T, NanoError>;

#[derive(Debug, Clone, PartialEq, FromPrimitive, Copy)]
pub enum NanoErrorKind {
    Unknown                    = 0isize,
    OperationNotSupported      = libnanomsg::ENOTSUP          as isize,
    ProtocolNotSupported       = libnanomsg::EPROTONOSUPPORT  as isize,
    NoBufferSpace              = libnanomsg::ENOBUFS          as isize,
    NetworkDown                = libnanomsg::ENETDOWN         as isize,
    AddressInUse               = libnanomsg::EADDRINUSE       as isize,
    AddressNotAvailable        = libnanomsg::EADDRNOTAVAIL    as isize,
    ConnectionRefused          = libnanomsg::ECONNREFUSED     as isize,
    OperationNowInProgress     = libnanomsg::EINPROGRESS      as isize,
    NotSocket                  = libnanomsg::ENOTSOCK         as isize,
    AddressFamilyNotSupported  = libnanomsg::EAFNOSUPPORT     as isize,
    WrongProtocol              = libnanomsg::EPROTO           as isize,
    TryAgain                   = libnanomsg::EAGAIN           as isize,
    BadFileDescriptor          = libnanomsg::EBADF            as isize,
    InvalidArgument            = libnanomsg::EINVAL           as isize,
    TooManyOpenFiles           = libnanomsg::EMFILE           as isize,
    BadAddress                 = libnanomsg::EFAULT           as isize,
    PermisionDenied            = libnanomsg::EACCESS          as isize,
    NetworkReset               = libnanomsg::ENETRESET        as isize,
    NetworkUnreachable         = libnanomsg::ENETUNREACH      as isize,
    HostUnreachable            = libnanomsg::EHOSTUNREACH     as isize,
    NotConnected               = libnanomsg::ENOTCONN         as isize,
    MessageTooLong             = libnanomsg::EMSGSIZE         as isize,
    Timeout                    = libnanomsg::ETIMEDOUT        as isize,
    ConnectionAbort            = libnanomsg::ECONNABORTED     as isize,
    ConnectionReset            = libnanomsg::ECONNRESET       as isize,
    ProtocolNotAvailable       = libnanomsg::ENOPROTOOPT      as isize,
    AlreadyConnected           = libnanomsg::EISCONN          as isize,
    SocketTypeNotSupported     = libnanomsg::ESOCKTNOSUPPORT  as isize,
    Terminating                = libnanomsg::ETERM            as isize,
    NameTooLong                = libnanomsg::ENAMETOOLONG     as isize,
    NoDevice                   = libnanomsg::ENODEV           as isize,
    FileStateMismatch          = libnanomsg::EFSM             as isize,
    Interrupted                = libnanomsg::EINTR            as isize
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
        let maybe_error_kind = FromPrimitive::from_isize(nn_errno as isize);
        let error_kind = maybe_error_kind.unwrap_or(Unknown);

        unsafe {
            let c_ptr: *const libc::c_char = libnanomsg::nn_strerror(nn_errno);
            let c_str = CStr::from_ptr(c_ptr);
            let bytes = c_str.to_bytes();
            let desc = str::from_utf8(bytes).unwrap_or("Error");

            NanoError::new(desc, error_kind)
        }
    }
}

impl FromError<io::Error> for NanoError {
    fn from_error(err: io::Error) -> NanoError {
        match err.kind() {
            io::ErrorKind::TimedOut                       => NanoError::from_nn_errno(libnanomsg::ETIMEDOUT),
            io::ErrorKind::InvalidInput                   => NanoError::from_nn_errno(libnanomsg::EINVAL),
            io::ErrorKind::FileNotFound                   => NanoError::from_nn_errno(libnanomsg::EBADF),
            io::ErrorKind::MismatchedFileTypeForOperation => NanoError::from_nn_errno(libnanomsg::ENOTSUP),
            io::ErrorKind::ResourceUnavailable            => NanoError::from_nn_errno(libnanomsg::EFSM),
            io::ErrorKind::Interrupted                    => NanoError::from_nn_errno(libnanomsg::EINTR),
            _                                             => NanoError::new("Other", Unknown)
        }
    }
}

impl FromError<NanoError> for io::Error {
    fn from_error(err: NanoError) -> io::Error {
        match err.kind {
            NanoErrorKind::Timeout                => io::Error::new(io::ErrorKind::TimedOut,                       err.description, None ),
            NanoErrorKind::InvalidArgument        => io::Error::new(io::ErrorKind::InvalidInput,                   err.description, None ),
            NanoErrorKind::BadFileDescriptor      => io::Error::new(io::ErrorKind::FileNotFound,                   err.description, None ),
            NanoErrorKind::OperationNotSupported  => io::Error::new(io::ErrorKind::MismatchedFileTypeForOperation, err.description, None ),
            NanoErrorKind::FileStateMismatch      => io::Error::new(io::ErrorKind::ResourceUnavailable,            err.description, None ),
            NanoErrorKind::Interrupted            => io::Error::new(io::ErrorKind::Interrupted,                    err.description, None ),
            _                                     => io::Error::new(io::ErrorKind::Other,                          err.description, None )
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
    use std::io;
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

    fn check_error_kind_match(nano_err_kind: NanoErrorKind, io_err_kind: io::ErrorKind) {
        let nano_err = NanoError::from_nn_errno(nano_err_kind as libc::c_int);
        let io_err: io::Error = FromError::from_error(nano_err);

        assert_eq!(io_err_kind, io_err.kind())
    }

    #[test]
    fn check_to_ioerror() {
        check_error_kind_match(NanoErrorKind::Timeout, io::ErrorKind::TimedOut);
        check_error_kind_match(NanoErrorKind::InvalidArgument, io::ErrorKind::InvalidInput);
        check_error_kind_match(NanoErrorKind::BadFileDescriptor, io::ErrorKind::FileNotFound);
        check_error_kind_match(NanoErrorKind::OperationNotSupported, io::ErrorKind::MismatchedFileTypeForOperation);
        check_error_kind_match(NanoErrorKind::FileStateMismatch, io::ErrorKind::ResourceUnavailable);
        check_error_kind_match(NanoErrorKind::Interrupted, io::ErrorKind::Interrupted);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out", None);
        let nano_err: NanoError = FromError::from_error(io_err);

        assert_eq!(NanoErrorKind::Timeout, nano_err.kind)
    }
}
