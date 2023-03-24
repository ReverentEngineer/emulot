use core::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    EncodingError,
    DaemonError,
    StorageError,
    QMPError,
    AlreadyRunning,
    AlreadyStopped,
    AlreadyExists,
    NoSuchEntity,
    Pending
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IOError => write!(f, "Input/output error"),
            ErrorKind::EncodingError => write!(f, "Encoding error"),
            ErrorKind::DaemonError => write!(f, "Daemon error"),
            ErrorKind::StorageError => write!(f, "Storage error"),
            ErrorKind::QMPError => write!(f, "QMP error"),
            ErrorKind::AlreadyRunning => write!(f, "Already running."),
            ErrorKind::AlreadyStopped => write!(f, "Already stopped."),
            ErrorKind::AlreadyExists => write!(f, "Already exists"),
            ErrorKind::NoSuchEntity => write!(f, "No such entity"),
            ErrorKind::Pending => write!(f, "Pending")
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {

    pub fn new<S: AsRef<str>>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.as_ref().to_string()
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0}", self.message)
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

impl From<curl::Error> for Error {

    fn from(error: curl::Error) -> Self {
        Error::new(ErrorKind::IOError, format!("{error}"))
    }

}

impl From<toml::de::Error> for Error {

    fn from(error: toml::de::Error) -> Self {
        Error::new(ErrorKind::IOError, format!("{error}"))
    }

}

impl From<hyper::Error> for Error {

    fn from(error: hyper::Error) -> Self {
        Error::new(ErrorKind::DaemonError, format!("{error}"))
    }

}

const SQLITE_CONSTRAINT_UNIQUE: i32 = 2067;

impl From<rusqlite::Error> for Error {

    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::SqliteFailure(kind, message) => {
                match kind {
                    kind if kind.extended_code == SQLITE_CONSTRAINT_UNIQUE => {
                        Error::new(ErrorKind::AlreadyExists, format!("{}", message.unwrap()))
                    },
                    _ => Error::new(ErrorKind::StorageError, format!("{}", message.unwrap()))
                }
            },
            rusqlite::Error::QueryReturnedNoRows =>
                Error::new(ErrorKind::NoSuchEntity, format!("{error}")),
            _ => Error::new(ErrorKind::StorageError, format!("{error}"))
        }
    }

}
