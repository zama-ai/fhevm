use std::io;
use tokio::sync::mpsc::UnboundedSender;
use tracing_subscriber::fmt::MakeWriter;

/// Custom `tracing` writer for tests.
///
/// It both prints every trace, and sends them to its channel for later analysis.
///
/// Heavily inspired by the `tracing-subscriber::fmt::TestWriter`.
pub struct CustomTestWriter {
    log_tx: UnboundedSender<Vec<u8>>,
}

impl CustomTestWriter {
    pub fn new(log_tx: UnboundedSender<Vec<u8>>) -> Self {
        Self { log_tx }
    }
}

impl io::Write for CustomTestWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.log_tx
            .send(buf.to_vec())
            .expect("log channel was closed!");

        let out_str = String::from_utf8_lossy(buf);
        print!("{out_str}");

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
