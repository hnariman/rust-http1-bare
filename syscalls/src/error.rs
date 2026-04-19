use core::fmt;

#[derive(Debug)]
pub enum HttpError {
    CreateSocketError(i32),
    BindSocketError(i32),
    ListenError(i32),
    AcceptError(i32),
    ReadSocketError(i32),
    WriteSocketError(i32),
    CloseSocketError(i32),
    ReuseSocketError(i32),
    NoActiveConnection,
}

impl HttpError {
    fn unpack_err(&self, code: &i32) -> &'static str {
        match code.abs() {
            9 => "EBADF : Bad file descriptor",
            13 => "EACCESS : Permission denied",
            98 => "EADDRINUSE : Address already in use",
            22 => "EINVAL : Invalid argument",
            24 => "EMPFILE : Too many open files",
            32 => "EPIPE : Broken pipe",
            _ => "unknown error, check with : errno -l {code}",
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateSocketError(code) => {
                write!(f, "Socket creation failed: {}", self.unpack_err(code))
            }
            Self::BindSocketError(code) => write!(f, "Binding failed: {}", self.unpack_err(code)),
            Self::ListenError(code) => write!(f, "Listen failed: {}", self.unpack_err(code)),
            Self::AcceptError(code) => write!(f, "Accept failed: {}", self.unpack_err(code)),
            Self::ReadSocketError(code) => write!(f, "Reading failed: {}", self.unpack_err(code)),
            Self::WriteSocketError(code) => write!(f, "Writing failed: {}", self.unpack_err(code)),
            Self::ReuseSocketError(code) => {
                write!(f, "Reuse socket failed: {}", self.unpack_err(code))
            }
            Self::CloseSocketError(code) => {
                write!(f, "Close socket failed: {}", self.unpack_err(code))
            }
            Self::NoActiveConnection => write!(f, "No active connection to operate on"),
        }
    }
}

pub trait AsmResult {
    fn or_err<F>(self, err_fn: F) -> Result<i32, HttpError>
    where
        F: FnOnce(i32) -> HttpError;
}

impl AsmResult for i32 {
    #[inline(always)]
    fn or_err<F>(self, err_fn: F) -> Result<i32, HttpError>
    where
        F: FnOnce(i32) -> HttpError,
    {
        if self < 0 {
            Err(err_fn(self))
        } else {
            Ok(self)
        }
    }
}
