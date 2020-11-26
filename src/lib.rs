#![warn(clippy::all, clippy::style, clippy::pedantic)]
#![allow(clippy::map_err_ignore)]

use std::fs::File;
use std::io;

use tokio_serde::formats::Bincode;

pub mod config_firewall;
pub mod models;
pub mod rpc;

/// Utility function to open a file with a `BufReader` and then pass the `BufReader` into a method
///
/// # Errors
/// if opening the file fails or if `F` returns an error
pub fn open_file_with<F, T>(file: &str, method: F) -> io::Result<T>
where
    F: FnOnce(&mut dyn io::BufRead) -> Result<T, ()>,
{
    let certfile = File::open(file)?;
    let mut reader = io::BufReader::new(certfile);
    Ok(method(&mut reader).map_err(|_| io::Error::from(io::ErrorKind::Other))?)
}

/// Returns the codec used for RPC communication
#[must_use]
pub fn codec<Item, SinkItem>() -> fn() -> Bincode<Item, SinkItem> {
    Bincode::default
}
