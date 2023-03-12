use tokio::process::{
    Command,
    Child
};
use crate::{
    Error,
    ErrorKind,
    config::GuestConfig
};

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
            let mut command = Into::<Command>::into(self.config.as_cmd()); 
            self.process = Some(command.spawn()?);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::AlreadyRunning, format!("Already running")))
        }
    }

    pub async fn stop(&mut self) -> Result<(), Error> {
        self.process
            .take()
            .ok_or(Error::new(ErrorKind::AlreadyStopped, format!("Already stopped")))?
            .kill()
            .await?;
        self.process = None;
        Ok(())
    }

    pub async fn wait(&mut self) -> Result<std::process::ExitStatus, Error> {
        if let Some(ref mut process) = &mut self.process {
            Ok(process.wait().await?)
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

