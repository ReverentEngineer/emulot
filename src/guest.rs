use core::fmt;
use std::{
    process::Child,
    io
};
use crate::config::GuestConfig;

#[derive(Debug, PartialEq)]
enum ErrorKind {
    IOError,
    AlreadyRunning,
    AlreadyStopped
}

#[derive(Debug, PartialEq)]
pub struct Error {
    #[allow(dead_code)]
    kind: ErrorKind,
    message: String
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error {

    fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message
        }
    }

}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::new(ErrorKind::IOError, error.to_string())
    }
}

/// Status of Guest
#[derive(PartialEq)]
pub enum Status {
    Running,
    Stopped,
}

#[derive(Debug)]
pub struct Guest {
    config: GuestConfig,
    process: Option<Child>,
}

impl From<Guest> for GuestConfig {

    fn from(guest: Guest) -> Self {
        guest.config
    }

}

impl Guest {

    pub fn run(&mut self) -> Result<(), Error> {
        if self.status()? != Status::Running {
            self.process = Some(self.config.as_cmd().spawn()?);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::AlreadyRunning, format!("Already running")))
        }
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) -> Result<(), Error> {
        self.process.as_mut()
            .ok_or(Error::new(ErrorKind::AlreadyStopped, format!("Already stopped")))?
            .kill()?;
        Ok(())
    }

    pub fn wait(&mut self) -> Result<std::process::ExitStatus, Error> {
        if let Some(ref mut process) = &mut self.process {
            Ok(process.wait()?)
        } else {
            Err(Error::new(ErrorKind::AlreadyStopped, format!("Already stopped")))
        }
    }

    pub fn status(&mut self) -> Result<Status, Error> {
        match &mut self.process {
            Some(process) => {
                if process.try_wait()? == None {
                    Ok(Status::Running)
                } else {
                    Ok(Status::Stopped)
                }
            }
            None => Ok(Status::Stopped),
        }
    }
}

impl From<GuestConfig> for Guest {
    fn from(config: GuestConfig) -> Self {
        Self {
            config,
            process: None
        }
    }
}

