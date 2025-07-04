use core::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    SystemTimeError,
    HarnessError,
    StorageError,
    AlreadyExists,
    NoSuchEntity
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IOError => write!(f, "Input/output error"),
            ErrorKind::SystemTimeError => write!(f, "System time error"),
            ErrorKind::HarnessError => write!(f, "Harness error"),
            ErrorKind::StorageError => write!(f, "Storage error"),
            ErrorKind::AlreadyExists => write!(f, "Already exists"),
            ErrorKind::NoSuchEntity => write!(f, "No such entity"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new<S: AsRef<str>>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.as_ref().to_string(),
        }
    }

}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = &self.message;
        match self.kind {
            ErrorKind::IOError => write!(f, "i/o error: {message}"),
            ErrorKind::SystemTimeError => write!(f, "{message}"),
            ErrorKind::HarnessError => write!(f, "harness error: {message}"),
            ErrorKind::StorageError => write!(f, "storage error: {message}"),
            ErrorKind::AlreadyExists => write!(f, "Already exists."),
            ErrorKind::NoSuchEntity => write!(f, "No such entity.")
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(ErrorKind::IOError, format!("{error}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::new(ErrorKind::IOError, format!("{error}"))
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::new(ErrorKind::IOError, format!("{error}"))
    }
}

const SQLITE_CONSTRAINT_UNIQUE: i32 = 2067;

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::SqliteFailure(kind, message) => match kind {
                kind if kind.extended_code == SQLITE_CONSTRAINT_UNIQUE => {
                    Error::new(ErrorKind::AlreadyExists, message.unwrap())
                }
                _ => Error::new(ErrorKind::StorageError, message.unwrap()),
            },
            rusqlite::Error::QueryReturnedNoRows => {
                Error::new(ErrorKind::NoSuchEntity, format!("{error}"))
            }
            _ => Error::new(ErrorKind::StorageError, format!("{error}")),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Self::new(ErrorKind::SystemTimeError, format!("{error}"))
    }
}

impl From<system_harness::Error> for Error {
    fn from(error: system_harness::Error) -> Self {
        Self::new(ErrorKind::HarnessError, format!("{error}"))
    }
}
