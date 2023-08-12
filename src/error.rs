use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("response is missing content")]
    ContentMissingError,
    #[error("{0}")]
    UnspecifiedError(String),
}

#[macro_export]
macro_rules! bail {
    () => {
        return Err($crate::error::SPError::UnspecifiedError("unknown error".to_string()))
    };
    ($msg:literal $(,)?) => {
        return Err($crate::error::SPError::UnspecifiedError($msg.to_string()))
    };
    ($err:expr $(,)?) => {
        return Err($crate::error::SPError::UnspecifiedError(($err).to_string()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::SPError::UnspecifiedError(format!($fmt, $($arg)*)))
    };
}
