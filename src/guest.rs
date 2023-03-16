use std::process::Stdio;
use tokio::{
    io::BufReader,
    process::{
        Child,
        ChildStdin,
        ChildStdout
    }
};
use crate::{
    Error,
    ErrorKind,
    qmp::{
        Command,
        Execute,
        Response,
        AsyncSend,
        AsyncReceive
    },
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
    writer: Option<ChildStdin>,
    reader: Option<BufReader<ChildStdout>>
}

impl From<Guest> for GuestConfig {

    fn from(guest: Guest) -> Self {
        guest.config
    }

}

impl Guest {

    pub async fn run(&mut self) -> Result<(), Error> {
        if self.status()? != Status::Running {
            let mut command = Into::<tokio::process::Command>::into(self.config.as_cmd()); 
            command.stdin(Stdio::piped())
                .stdout(Stdio::piped());
            let mut child = command.spawn()?;
            if let Some(reader) = child.stdout.take() {
                let mut reader = BufReader::new(reader);
                match reader.receive().await? {
                    Response::Greeting(_) => (),
                    _ => return Err(Error::new(ErrorKind::IOError, format!("No greeting received.")))
                }
                if let Some(mut writer) = child.stdin.take() {
                    writer.send(Command {
                        execute: Execute::QmpCapabilities
                    })?.await?;
                    match reader.receive().await? {
                        Response::Return(_) => {
                            self.writer = Some(writer);
                            self.reader = Some(reader);
                            self.process = Some(child);
                            Ok(())
                        },
                        _ => Err(Error::new(ErrorKind::IOError, format!("Unexpected message received")))
                    }
                } else {
                    Err(Error::new(ErrorKind::IOError, format!("Failed to communicate with guest")))
                }

            } else {
                Err(Error::new(ErrorKind::IOError, format!("Failed to communicate with guest")))
            }
        } else {
            Err(Error::new(ErrorKind::AlreadyRunning, format!("Already running")))
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        self.send(Command {
            execute: Execute::SystemPowerDown
        }).await?;
        let _ = self.wait_for_return().await?;
        Ok(())
    }

    async fn wait_for_return(&mut self) -> Result<Response, Error> {
        while match self.receive().await? {
            Response::Return(contents) => return Ok(Response::Return(contents)),
            Response::Error(_) => false,
            _ => true,
        } { 
        }
        unreachable!()
    }

    async fn send(&mut self, command: Command) -> Result<(), Error> {
        if let Some(ref mut writer) = self.writer {
            writer.send(command)?.await
        } else {
            Err(Error::new(ErrorKind::IOError, format!("Failed to communicate with guest")))
        }
    }

    async fn receive(&mut self) -> Result<Response, Error> {
        if let Some(ref mut reader) = self.reader {
            reader.receive().await
        } else {
            Err(Error::new(ErrorKind::IOError, format!("Failed to communicate with guest")))
        }
    }

    pub async fn kill(&mut self) -> Result<(), Error> {
        self.process
            .take()
            .ok_or(Error::new(ErrorKind::AlreadyStopped, format!("Already stopped")))?
            .kill()
            .await?;
        self.writer = None;
        self.reader = None;
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
            process: None,
            reader: None,
            writer: None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn run() {
        let config = GuestConfig::new("x86_64".to_string(), 512);
        let mut guest = Into::<Guest>::into(config);
        guest.run().await.unwrap();
        guest.shutdown().await.unwrap();
        guest.kill().await.unwrap();
    }


}
