//! Contains error type for handling I/O and Errno errors
#[cfg(unix)]
use nix;
#[cfg(windows)]
use std::char;
use std::error;
use std::fmt;
use std::io;
use std::str;

/// The error type for Rustyline errors that can arise from
/// I/O related errors or Errno when using the nix-rust library
// #[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum ReadlineError {
    /// I/O Error
    Io(io::Error),
    /// EOF (Ctrl-D)
    Eof,
    /// Ctrl-C
    Interrupted,
    /// Ctrl-Z
    #[cfg(unix)]
    Suspended,
    /// Chars Error
    #[cfg(unix)]
    Utf8Error,
    /// Unix Error from syscall
    #[cfg(unix)]
    Errno(nix::Error),
    #[cfg(windows)]
    WindowResize,
    #[cfg(windows)]
    Decode(char::DecodeUtf16Error),
}

impl fmt::Display for ReadlineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReadlineError::Io(ref err) => err.fmt(f),
            ReadlineError::Eof => write!(f, "EOF"),
            ReadlineError::Interrupted => write!(f, "Interrupted"),
            #[cfg(unix)]
            ReadlineError::Suspended => write!(f, "Suspended"),
            #[cfg(unix)]
            ReadlineError::Utf8Error => write!(f, "invalid utf-8: corrupt contents"),
            #[cfg(unix)]
            ReadlineError::Errno(ref err) => err.fmt(f),
            #[cfg(windows)]
            ReadlineError::WindowResize => write!(f, "WindowResize"),
            #[cfg(windows)]
            ReadlineError::Decode(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for ReadlineError {
    fn description(&self) -> &str {
        match *self {
            ReadlineError::Io(ref err) => err.description(),
            ReadlineError::Eof => "EOF",
            ReadlineError::Interrupted => "Interrupted",
            #[cfg(unix)]
            ReadlineError::Suspended => "Suspended",
            #[cfg(unix)]
            ReadlineError::Utf8Error => "invalid utf-8: corrupt contents",
            #[cfg(unix)]
            ReadlineError::Errno(ref err) => err.description(),
            #[cfg(windows)]
            ReadlineError::WindowResize => "WindowResize",
            #[cfg(windows)]
            ReadlineError::Decode(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for ReadlineError {
    fn from(err: io::Error) -> Self {
        ReadlineError::Io(err)
    }
}

#[cfg(unix)]
impl From<nix::Error> for ReadlineError {
    fn from(err: nix::Error) -> Self {
        ReadlineError::Errno(err)
    }
}

#[cfg(windows)]
impl From<char::DecodeUtf16Error> for ReadlineError {
    fn from(err: char::DecodeUtf16Error) -> Self {
        ReadlineError::Decode(err)
    }
}
