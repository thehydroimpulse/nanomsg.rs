extern crate libc;

use libc::{c_int, c_char};
use nanomsg_sys;

use std::str;
use std::fmt;
use std::io;
use std::convert::From;
use std::num::FromPrimitive;
use std::ffi::CStr;

pub use self::NanoErrorKind::*;

pub type NanoResult<T> = Result<T, NanoError>;

#[derive(Debug, Clone, PartialEq, FromPrimitive, Copy)]
pub enum NanoErrorKind {
    Unknown                    = 0isize,
    OperationNotSupported      = nanomsg_sys::ENOTSUP          as isize,
    ProtocolNotSupported       = nanomsg_sys::EPROTONOSUPPORT  as isize,
    NoBufferSpace              = nanomsg_sys::ENOBUFS          as isize,
    NetworkDown                = nanomsg_sys::ENETDOWN         as isize,
    AddressInUse               = nanomsg_sys::EADDRINUSE       as isize,
    AddressNotAvailable        = nanomsg_sys::EADDRNOTAVAIL    as isize,
    ConnectionRefused          = nanomsg_sys::ECONNREFUSED     as isize,
    OperationNowInProgress     = nanomsg_sys::EINPROGRESS      as isize,
    NotSocket                  = nanomsg_sys::ENOTSOCK         as isize,
    AddressFamilyNotSupported  = nanomsg_sys::EAFNOSUPPORT     as isize,
    WrongProtocol              = nanomsg_sys::EPROTO           as isize,
    TryAgain                   = nanomsg_sys::EAGAIN           as isize,
    BadFileDescriptor          = nanomsg_sys::EBADF            as isize,
    InvalidArgument            = nanomsg_sys::EINVAL           as isize,
    TooManyOpenFiles           = nanomsg_sys::EMFILE           as isize,
    BadAddress                 = nanomsg_sys::EFAULT           as isize,
    PermissionDenied           = nanomsg_sys::EACCESS          as isize,
    NetworkReset               = nanomsg_sys::ENETRESET        as isize,
    NetworkUnreachable         = nanomsg_sys::ENETUNREACH      as isize,
    HostUnreachable            = nanomsg_sys::EHOSTUNREACH     as isize,
    NotConnected               = nanomsg_sys::ENOTCONN         as isize,
    MessageTooLong             = nanomsg_sys::EMSGSIZE         as isize,
    TimedOut                   = nanomsg_sys::ETIMEDOUT        as isize,
    ConnectionAbort            = nanomsg_sys::ECONNABORTED     as isize,
    ConnectionReset            = nanomsg_sys::ECONNRESET       as isize,
    ProtocolNotAvailable       = nanomsg_sys::ENOPROTOOPT      as isize,
    AlreadyConnected           = nanomsg_sys::EISCONN          as isize,
    SocketTypeNotSupported     = nanomsg_sys::ESOCKTNOSUPPORT  as isize,
    Terminating                = nanomsg_sys::ETERM            as isize,
    NameTooLong                = nanomsg_sys::ENAMETOOLONG     as isize,
    NoDevice                   = nanomsg_sys::ENODEV           as isize,
    FileStateMismatch          = nanomsg_sys::EFSM             as isize,
    Interrupted                = nanomsg_sys::EINTR            as isize
}

#[derive(PartialEq, Clone, Copy)]
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
            let c_ptr: *const libc::c_char = nanomsg_sys::nn_strerror(nn_errno);
            let c_str = CStr::from_ptr(c_ptr);
            let bytes = c_str.to_bytes();
            let desc = str::from_utf8(bytes).unwrap_or("Error");

            NanoError::new(desc, error_kind)
        }
    }
}

impl From<io::Error> for NanoError {
    fn from(err: io::Error) -> NanoError {
        match err.kind() {
            io::ErrorKind::PermissionDenied    => NanoError::from_nn_errno(nanomsg_sys::EACCESS),
            io::ErrorKind::ConnectionRefused   => NanoError::from_nn_errno(nanomsg_sys::ECONNREFUSED),
            io::ErrorKind::ConnectionReset     => NanoError::from_nn_errno(nanomsg_sys::ECONNRESET),
            io::ErrorKind::ConnectionAborted   => NanoError::from_nn_errno(nanomsg_sys::ECONNABORTED),
            io::ErrorKind::NotConnected        => NanoError::from_nn_errno(nanomsg_sys::ENOTCONN),
            io::ErrorKind::AddrInUse           => NanoError::from_nn_errno(nanomsg_sys::EADDRINUSE),
            io::ErrorKind::AddrNotAvailable    => NanoError::from_nn_errno(nanomsg_sys::EADDRNOTAVAIL),
            io::ErrorKind::AlreadyExists       => NanoError::from_nn_errno(nanomsg_sys::EISCONN),
            io::ErrorKind::WouldBlock          => NanoError::from_nn_errno(nanomsg_sys::EAGAIN),
            io::ErrorKind::InvalidInput        => NanoError::from_nn_errno(nanomsg_sys::EINVAL),
            io::ErrorKind::TimedOut            => NanoError::from_nn_errno(nanomsg_sys::ETIMEDOUT),
            io::ErrorKind::Interrupted         => NanoError::from_nn_errno(nanomsg_sys::EINTR),
            _                                  => NanoError::new("Other", Unknown)
        }
    }
}

impl From<NanoError> for io::Error {
    fn from(err: NanoError) -> io::Error {
        match err.kind {
            NanoErrorKind::PermissionDenied      => io::Error::new(io::ErrorKind::PermissionDenied,  err.description ),
            NanoErrorKind::ConnectionRefused     => io::Error::new(io::ErrorKind::ConnectionRefused, err.description ),
            NanoErrorKind::ConnectionReset       => io::Error::new(io::ErrorKind::ConnectionReset,   err.description ),
            NanoErrorKind::ConnectionAbort       => io::Error::new(io::ErrorKind::ConnectionAborted, err.description ),
            NanoErrorKind::NotConnected          => io::Error::new(io::ErrorKind::NotConnected,      err.description ),
            NanoErrorKind::AddressInUse          => io::Error::new(io::ErrorKind::AddrInUse,         err.description ),
            NanoErrorKind::AddressNotAvailable   => io::Error::new(io::ErrorKind::AddrNotAvailable,  err.description ),
            NanoErrorKind::AlreadyConnected      => io::Error::new(io::ErrorKind::AlreadyExists,     err.description ),
            NanoErrorKind::TryAgain              => io::Error::new(io::ErrorKind::WouldBlock,        err.description ),
            NanoErrorKind::InvalidArgument       => io::Error::new(io::ErrorKind::InvalidInput,      err.description ),
            NanoErrorKind::TimedOut              => io::Error::new(io::ErrorKind::TimedOut,          err.description ),
            NanoErrorKind::Interrupted           => io::Error::new(io::ErrorKind::Interrupted,       err.description ),
            _                                    => io::Error::new(io::ErrorKind::Other,             err.description )
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
    let nn_errno = unsafe { nanomsg_sys::nn_errno() };

    NanoError::from_nn_errno(nn_errno)
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use nanomsg_sys;
    use libc;
    use super::NanoErrorKind::*;
    use super::NanoErrorKind;
    use super::NanoError;
    use std::io;
    use std::convert::From;
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
        assert_convert_error_code_to_error_kind(nanomsg_sys::ENOTSUP, OperationNotSupported);
        assert_convert_error_code_to_error_kind(nanomsg_sys::EPROTONOSUPPORT, ProtocolNotSupported);
        assert_convert_error_code_to_error_kind(nanomsg_sys::EADDRINUSE, AddressInUse);
        assert_convert_error_code_to_error_kind(nanomsg_sys::EHOSTUNREACH, HostUnreachable);
    }

    fn check_error_kind_match(nano_err_kind: NanoErrorKind, io_err_kind: io::ErrorKind) {
        let nano_err = NanoError::from_nn_errno(nano_err_kind as libc::c_int);
        let io_err: io::Error = From::from(nano_err);

        assert_eq!(io_err_kind, io_err.kind())
    }

    #[test]
    fn check_to_ioerror() {
        check_error_kind_match(NanoErrorKind::TimedOut, io::ErrorKind::TimedOut);
        check_error_kind_match(NanoErrorKind::PermissionDenied, io::ErrorKind::PermissionDenied);
        check_error_kind_match(NanoErrorKind::ConnectionRefused, io::ErrorKind::ConnectionRefused);
        check_error_kind_match(NanoErrorKind::OperationNotSupported, io::ErrorKind::Other);
        check_error_kind_match(NanoErrorKind::NotConnected, io::ErrorKind::NotConnected);
        check_error_kind_match(NanoErrorKind::Interrupted, io::ErrorKind::Interrupted);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
        let nano_err: NanoError = From::from(io_err);

        assert_eq!(NanoErrorKind::TimedOut, nano_err.kind)
    }
}
