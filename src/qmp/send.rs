use core::{
    future::Future,
    task::{Poll, Context},
    pin::Pin
};
use tokio::io::AsyncWrite;
use crate::{
    qmp::Command,
    Error,
    ErrorKind
};
use pin_project_lite::pin_project;

/// Can send a QMP command
pub trait AsyncSend: AsyncWrite {

    /// Send a command asynchronously
    fn send<'a>(&'a mut self, command: Command) -> Result<SendCommand<'a, Self>, Error>
    where
        Self: Sized + Unpin {
        Ok(SendCommand {
            writer: self,
            command: serde_json::to_vec(&command)?,
            sent: 0
        })
    }

}

impl<W> AsyncSend for W where W: AsyncWrite { }

pin_project! {

    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct SendCommand<'a, W> {
        writer: &'a mut W,
        command: Vec<u8>,
        sent: usize
    }

}

impl<W> Future for SendCommand<'_, W>
where
    W: AsyncWrite + Unpin
{
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        while *me.sent <  me.command.len() {
            let n = match Pin::new(&mut *me.writer).poll_write(cx, &me.command[*me.sent..]) {
                Poll::Ready(Ok(0)) => return Poll::Ready(Err(Error::new(ErrorKind::IOError, "No data written"))),
                Poll::Ready(Ok(written)) => written,
                Poll::Ready(Err(err)) => return Poll::Ready(Err(err.into())),
                Poll::Pending => return Poll::Pending,
            };
            *me.sent = *me.sent + n;
        }
        Poll::Ready(Ok(()))
    }
}
