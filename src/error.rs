use std::{
    error,
    fmt,
    result
};

#[derive(Clone, Debug)]
pub struct WinfetchError(pub String);

impl fmt::Display for WinfetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for WinfetchError { }

pub type WinfetchResult<T> = result::Result<T, WinfetchError>;
