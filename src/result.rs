extern crate libc;

use libc::c_int;

use std::convert::From;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Error {
    Unknown = 0 as isize,
    OperationNotSupported = nanomsg_sys::ENOTSUP as isize,
    ProtocolNotSupported = nanomsg_sys::EPROTONOSUPPORT as isize,
    NoBufferSpace = nanomsg_sys::ENOBUFS as isize,
    NetworkDown = nanomsg_sys::ENETDOWN as isize,
    AddressInUse = nanomsg_sys::EADDRINUSE as isize,
    AddressNotAvailable = nanomsg_sys::EADDRNOTAVAIL as isize,
    ConnectionRefused = nanomsg_sys::ECONNREFUSED as isize,
    OperationNowInProgress = nanomsg_sys::EINPROGRESS as isize,
    NotSocket = nanomsg_sys::ENOTSOCK as isize,
    AddressFamilyNotSupported = nanomsg_sys::EAFNOSUPPORT as isize,
    #[cfg(not(target_os = "openbsd"))]
    WrongProtocol = nanomsg_sys::EPROTO as isize,
    #[cfg(target_os = "openbsd")]
    WrongProtocol = nanomsg_sys::EPROTOTYPE as isize,
    TryAgain = nanomsg_sys::EAGAIN as isize,
    BadFileDescriptor = nanomsg_sys::EBADF as isize,
    InvalidInput = nanomsg_sys::EINVAL as isize,
    TooManyOpenFiles = nanomsg_sys::EMFILE as isize,
    BadAddress = nanomsg_sys::EFAULT as isize,
    PermissionDenied = nanomsg_sys::EACCESS as isize,
    NetworkReset = nanomsg_sys::ENETRESET as isize,
    NetworkUnreachable = nanomsg_sys::ENETUNREACH as isize,
    HostUnreachable = nanomsg_sys::EHOSTUNREACH as isize,
    NotConnected = nanomsg_sys::ENOTCONN as isize,
    MessageTooLong = nanomsg_sys::EMSGSIZE as isize,
    TimedOut = nanomsg_sys::ETIMEDOUT as isize,
    ConnectionAborted = nanomsg_sys::ECONNABORTED as isize,
    ConnectionReset = nanomsg_sys::ECONNRESET as isize,
    ProtocolNotAvailable = nanomsg_sys::ENOPROTOOPT as isize,
    AlreadyConnected = nanomsg_sys::EISCONN as isize,
    SocketTypeNotSupported = nanomsg_sys::ESOCKTNOSUPPORT as isize,
    Terminating = nanomsg_sys::ETERM as isize,
    NameTooLong = nanomsg_sys::ENAMETOOLONG as isize,
    NoDevice = nanomsg_sys::ENODEV as isize,
    FileStateMismatch = nanomsg_sys::EFSM as isize,
    Interrupted = nanomsg_sys::EINTR as isize,
}

impl Error {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }

    pub fn from_raw(raw: c_int) -> Error {
        match raw {
            nanomsg_sys::ENOTSUP => Error::OperationNotSupported,
            nanomsg_sys::EPROTONOSUPPORT => Error::ProtocolNotSupported,
            nanomsg_sys::ENOBUFS => Error::NoBufferSpace,
            nanomsg_sys::ENETDOWN => Error::NetworkDown,
            nanomsg_sys::EADDRINUSE => Error::AddressInUse,
            nanomsg_sys::EADDRNOTAVAIL => Error::AddressNotAvailable,
            nanomsg_sys::ECONNREFUSED => Error::ConnectionRefused,
            nanomsg_sys::EINPROGRESS => Error::OperationNowInProgress,
            nanomsg_sys::ENOTSOCK => Error::NotSocket,
            nanomsg_sys::EAFNOSUPPORT => Error::AddressFamilyNotSupported,
            #[cfg(not(target_os = "openbsd"))]
            nanomsg_sys::EPROTO => Error::WrongProtocol,
            #[cfg(target_os = "openbsd")]
            nanomsg_sys::EPROTOTYPE => Error::WrongProtocol,
            nanomsg_sys::EAGAIN => Error::TryAgain,
            nanomsg_sys::EBADF => Error::BadFileDescriptor,
            nanomsg_sys::EINVAL => Error::InvalidInput,
            nanomsg_sys::EMFILE => Error::TooManyOpenFiles,
            nanomsg_sys::EFAULT => Error::BadAddress,
            nanomsg_sys::EACCESS => Error::PermissionDenied,
            nanomsg_sys::ENETRESET => Error::NetworkReset,
            nanomsg_sys::ENETUNREACH => Error::NetworkUnreachable,
            nanomsg_sys::EHOSTUNREACH => Error::HostUnreachable,
            nanomsg_sys::ENOTCONN => Error::NotConnected,
            nanomsg_sys::EMSGSIZE => Error::MessageTooLong,
            nanomsg_sys::ETIMEDOUT => Error::TimedOut,
            nanomsg_sys::ECONNABORTED => Error::ConnectionAborted,
            nanomsg_sys::ECONNRESET => Error::ConnectionReset,
            nanomsg_sys::ENOPROTOOPT => Error::ProtocolNotAvailable,
            nanomsg_sys::EISCONN => Error::AlreadyConnected,
            nanomsg_sys::ESOCKTNOSUPPORT => Error::SocketTypeNotSupported,
            nanomsg_sys::ETERM => Error::Terminating,
            nanomsg_sys::ENAMETOOLONG => Error::NameTooLong,
            nanomsg_sys::ENODEV => Error::NoDevice,
            nanomsg_sys::EFSM => Error::FileStateMismatch,
            nanomsg_sys::EINTR => Error::Interrupted,
            _ => Error::Unknown,
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        match err.kind() {
            io::ErrorKind::PermissionDenied => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused => Error::ConnectionRefused,
            io::ErrorKind::ConnectionReset => Error::ConnectionReset,
            io::ErrorKind::ConnectionAborted => Error::ConnectionAborted,
            io::ErrorKind::NotConnected => Error::NotConnected,
            io::ErrorKind::AddrInUse => Error::AddressInUse,
            io::ErrorKind::AddrNotAvailable => Error::AddressNotAvailable,
            io::ErrorKind::AlreadyExists => Error::AlreadyConnected,
            io::ErrorKind::WouldBlock => Error::TryAgain,
            io::ErrorKind::InvalidInput => Error::InvalidInput,
            io::ErrorKind::TimedOut => Error::TimedOut,
            io::ErrorKind::Interrupted => Error::Interrupted,
            _ => Error::Unknown,
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        let as_std_error: &dyn error::Error = &err;
        let description = as_std_error.to_string();
        match err {
            Error::PermissionDenied => io::Error::new(io::ErrorKind::PermissionDenied, description),
            Error::ConnectionRefused => {
                io::Error::new(io::ErrorKind::ConnectionRefused, description)
            }
            Error::ConnectionReset => io::Error::new(io::ErrorKind::ConnectionReset, description),
            Error::ConnectionAborted => {
                io::Error::new(io::ErrorKind::ConnectionAborted, description)
            }
            Error::NotConnected => io::Error::new(io::ErrorKind::NotConnected, description),
            Error::AddressInUse => io::Error::new(io::ErrorKind::AddrInUse, description),
            Error::AddressNotAvailable => {
                io::Error::new(io::ErrorKind::AddrNotAvailable, description)
            }
            Error::AlreadyConnected => io::Error::new(io::ErrorKind::AlreadyExists, description),
            Error::TryAgain => io::Error::new(io::ErrorKind::WouldBlock, description),
            Error::InvalidInput => io::Error::new(io::ErrorKind::InvalidInput, description),
            Error::TimedOut => io::Error::new(io::ErrorKind::TimedOut, description),
            Error::Interrupted => io::Error::new(io::ErrorKind::Interrupted, description),
            _ => io::Error::new(io::ErrorKind::Other, description),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let description = unsafe {
            let nn_errno = *self as c_int;
            let c_ptr: *const libc::c_char = nanomsg_sys::nn_strerror(nn_errno);
            let c_str = CStr::from_ptr(c_ptr);
            let bytes = c_str.to_bytes();
            str::from_utf8(bytes).unwrap_or("Error")
        };
        write!(formatter, "{}", description)
    }
}

pub fn last_nano_error() -> Error {
    let nn_errno = unsafe { nanomsg_sys::nn_errno() };

    Error::from_raw(nn_errno)
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::Error;
    use libc;
    use nanomsg_sys;
    use std::convert::From;
    use std::io;

    fn assert_convert_error_code_to_error(error_code: libc::c_int, expected_error: Error) {
        let converted_error = Error::from_raw(error_code);
        assert_eq!(expected_error, converted_error)
    }

    #[test]
    fn can_convert_error_code_to_error() {
        assert_convert_error_code_to_error(nanomsg_sys::ENOTSUP, Error::OperationNotSupported);
        assert_convert_error_code_to_error(
            nanomsg_sys::EPROTONOSUPPORT,
            Error::ProtocolNotSupported,
        );
        assert_convert_error_code_to_error(nanomsg_sys::EADDRINUSE, Error::AddressInUse);
        assert_convert_error_code_to_error(nanomsg_sys::EHOSTUNREACH, Error::HostUnreachable);
    }

    fn check_error_kind_match(nano_err: Error, io_err_kind: io::ErrorKind) {
        let io_err: io::Error = From::from(nano_err);

        assert_eq!(io_err_kind, io_err.kind())
    }

    #[test]
    fn nano_err_can_be_converted_to_io_err() {
        check_error_kind_match(Error::TimedOut, io::ErrorKind::TimedOut);
        check_error_kind_match(Error::PermissionDenied, io::ErrorKind::PermissionDenied);
        check_error_kind_match(Error::ConnectionRefused, io::ErrorKind::ConnectionRefused);
        check_error_kind_match(Error::OperationNotSupported, io::ErrorKind::Other);
        check_error_kind_match(Error::NotConnected, io::ErrorKind::NotConnected);
        check_error_kind_match(Error::Interrupted, io::ErrorKind::Interrupted);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
        let nano_err: Error = From::from(io_err);

        assert_eq!(Error::TimedOut, nano_err)
    }
}
