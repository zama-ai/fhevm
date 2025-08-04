use std::io;
use tokio::sync::mpsc::UnboundedSender;
use tracing_subscriber::fmt::MakeWriter;

/// Custom `tracing` writer for tests.
///
/// It both prints every trace, and sends them to its channel for later analysis.
///
/// Heavily inspired by the `tracing-subscriber::fmt::TestWriter`.
pub struct CustomTestWriter {
    log_tx: UnboundedSender<String>,
}

impl CustomTestWriter {
    pub fn new(log_tx: UnboundedSender<String>) -> Self {
        Self { log_tx }
    }
}

impl io::Write for CustomTestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let out_str = String::from_utf8_lossy(buf);
        print!("{out_str}");

        self.log_tx
            .send(out_str.to_string())
            .expect("log channel was closed!");

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for CustomTestWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        Self {
            log_tx: self.log_tx.clone(),
        }
    }
}
