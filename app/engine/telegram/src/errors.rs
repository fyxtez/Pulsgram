use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TelegramError {
    Invocation(grammers_client::InvocationError),
    SignIn(Box<grammers_client::SignInError>),
    StdIO(std::io::Error),
    Other(String),
    EnvVar {
        name: String,
        source: std::env::VarError,
    },

    ParseInt {
        name: String,
        source: std::num::ParseIntError,
    },
}

impl Display for TelegramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TelegramError::Invocation(msg) => write!(f, "Invocation error: {}", msg),
            TelegramError::StdIO(msg) => write!(f, "std::io error: {}", msg),
            TelegramError::SignIn(msg) => write!(f, "Telegram::SignIn error: {}", msg),
            TelegramError::Other(msg) => write!(f, "Other error: {}", msg),
            TelegramError::EnvVar { name, source } => {
                write!(f, "Environment variable '{}' error: {}", name, source)
            }
            TelegramError::ParseInt { name, source } => {
                write!(f, "Failed to parse '{}' as i32: {}", name, source)
            }
        }
    }
}

// The `source()` method enables error chaining by returning the
// underlying cause of this error when one exists.
//
// For variants that wrap another concrete error type
// (InvocationError, SignInError, std::io::Error),
// the inner error is returned so callers can inspect the original cause.
//
// The `Other` variant contains only a String and does not wrap
// another error, so it returns `None`.
impl Error for TelegramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TelegramError::Invocation(err) => Some(err),
            TelegramError::SignIn(err) => Some(err),
            TelegramError::StdIO(err) => Some(err),
            TelegramError::Other(_) => None,
            TelegramError::EnvVar { source, .. } => Some(source),
            TelegramError::ParseInt { source, .. } => Some(source),
        }
    }
}

impl From<grammers_client::InvocationError> for TelegramError {
    fn from(err: grammers_client::InvocationError) -> Self {
        TelegramError::Invocation(err)
    }
}

impl From<std::io::Error> for TelegramError {
    fn from(err: std::io::Error) -> Self {
        TelegramError::StdIO(err)
    }
}

impl From<grammers_client::SignInError> for TelegramError {
    fn from(err: grammers_client::SignInError) -> Self {
        TelegramError::SignIn(Box::new(err))
    }
}
