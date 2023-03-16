use core::{
    future::Future,
    task::{Poll, Context},
    pin::Pin
};
use crate::{
    qmp::Response,
    Error,
    ErrorKind
};
use tokio::io::AsyncBufRead;
use pin_project_lite::pin_project;

/// Can send a QMP command
pub trait AsyncReceive: AsyncBufRead {
    
    /// Receive a message asynchronously
    fn receive<'a>(&'a mut self) -> ReceiveResponse<'a, Self>
    where
        Self: Sized + Unpin {
        ReceiveResponse {
            reader: self
        }
    }


}

impl<R> AsyncReceive for R where R: AsyncBufRead { }

pin_project! {

    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReceiveResponse<'a, R> {
        reader: &'a mut R,
    }

}

impl<R> Future for ReceiveResponse<'_, R>
where
    R: AsyncBufRead + Unpin
{
    type Output = Result<Response, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        match Pin::new(&mut *me.reader).poll_fill_buf(cx) {
            Poll::Ready(Ok(buf)) => {
                match buf.into_iter().position(|&c| c == 0x0A) {
                    Some(index) => match serde_json::from_slice(&buf[0..index]) {
                        Ok(response) => {
                            Pin::new(&mut *me.reader).consume(index + 1);
                            match response {
                                Response::Error(err) => {
                                    let class = err.get("class").map(|s| s.clone()).unwrap_or("Unknown".to_string());
                                    let desc = err.get("desc").map(|s| s.clone()).unwrap_or("No description".to_string());
                                    Poll::Ready(Err(Error::new(ErrorKind::QMPError, format!("QMP {class}: {desc}"))))
                                },
                                response => Poll::Ready(Ok(response))
                            }
                        },
                        Err(error) if error.is_eof() => Poll::Pending,
                        Err(error) => Poll::Ready(Err(error.into()))
                    },
                    None => Poll::Pending
                }
            },
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
            Poll::Pending => Poll::Pending
        }
    }
}
