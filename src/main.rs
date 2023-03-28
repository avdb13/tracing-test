use std::io::Write;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use tracing::debug;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub type SyncStr = Arc<Mutex<Sender<String>>>;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    tracing_subscriber::registry()
        .with(EnvFilter::new("tracing_test=debug"))
        .with(
            fmt::layer()
                .compact()
                .without_time()
                .with_writer(move || Writer::new(tx.clone())),
        )
        .init();

    debug!("hello");
    debug!("from");
    debug!("main");

    while let r = rx.recv() {
        match r {
            Ok(s) => println!("{s}"),
            Err(e) => panic!(),
        }
    }
}

impl Writer {
    fn new(tx: SyncStr) -> Self {
        Self { tx }
    }
}

#[derive(Clone)]
pub struct Writer {
    tx: SyncStr,
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).unwrap();
        let tx = self.tx.lock().unwrap();
        tx.send(s.to_owned()).unwrap();

        Ok(s.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
