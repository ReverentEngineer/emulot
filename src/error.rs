use core::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    QMPError,
    AlreadyRunning,
    AlreadyStopped,
    AlreadyExists,
    NoSuchEntity,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IOError => write!(f, "Input/output error"),
            ErrorKind::QMPError => write!(f, "QMP error"),
            ErrorKind::AlreadyRunning => write!(f, "Already running."),
            ErrorKind::AlreadyStopped => write!(f, "Already stopped."),
            ErrorKind::AlreadyExists => write!(f, "Already exists"),
            ErrorKind::NoSuchEntity => write!(f, "No such entity")

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
        write!(f, "{0}: {1}", self.kind, self.message)
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
