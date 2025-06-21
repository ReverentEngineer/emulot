use crossterm::{
    terminal::enable_raw_mode,
    event::{
        Event,
        EventStream,
        KeyCode,
        KeyEvent,
        KeyEventKind
    }
};
use crossterm::event::KeyModifiers;
use futures::StreamExt;
use futures::FutureExt;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::select;
use system_harness::QemuSystemTerminal;
use std::io::Write;

use crate::Error;

pub async fn attach_to_stdin<'sys>(terminal: &mut QemuSystemTerminal<'sys>) -> Result<(), Error> {
    enable_raw_mode()?;
    let mut events = EventStream::new();
    let mut stdout = std::io::stdout();
    loop {
        let mut buf = [0; 4096];
        select! {
            res = terminal.read(&mut buf)  => {
                let bytes_read = res?;
                if bytes_read > 0 {
                    stdout.write_all(&buf[0..bytes_read])?;
                    stdout.flush()?;
                }
            },
            event = events.next().fuse() => {
                match event {
                    Some(Ok(Event::Key(KeyEvent {
                        code,
                        modifiers,
                        kind,
                        state: _
                    }))) => {
                        if kind == KeyEventKind::Press {
                            match code {
                                KeyCode::Enter => {
                                    terminal.write_all("\n".as_bytes()).await?;
                                },
                                KeyCode::Backspace => {
                                    terminal.write_all("\x7F".as_bytes()).await?;
                                },
                                KeyCode::Char(chr) => {
                                    if chr == 'c' && (modifiers & KeyModifiers::CONTROL) == KeyModifiers::CONTROL {
                                       break Ok(()) 
                                    } else {
                                        terminal.write_all(format!("{chr}").as_bytes()).await?;
                                    }
                                }
                                code => log::error!("{code:?}"),
                            };
                        }
                    },
                    _ => {}
                };
            }
        }
    }
}
