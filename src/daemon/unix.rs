use core::{
    pin::Pin,
    task::{Context, Poll}
};
use std::io;
use std::path::Path;
use tokio::net::{
    UnixListener,
    UnixStream
};
use hyper::server::accept::Accept;
use axum::BoxError;

pub struct UnixAcceptor {
    uds: UnixListener,
}

impl UnixAcceptor {

    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self>  {
        UnixListener::bind(path)
            .map(|uds| Self { uds })
    }

}

impl Accept for UnixAcceptor {
    type Conn = UnixStream;
    type Error = BoxError;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let (stream, _addr) = match self.uds.poll_accept(cx)? {
            Poll::Ready(result) => result,
            Poll::Pending => return Poll::Pending
        };
        Poll::Ready(Some(Ok(stream)))
    }
}
