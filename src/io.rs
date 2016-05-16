//! Traits defining **Asynchronous** IO
//!
//! This module contains the _async_ variants of the `Read` and `Write` traits.

use AwaitBox;

use std::io::Result;

/// The `ReadAsync` trait allows for reading bytes from a source **asynchronously**.
pub trait ReadAsync {
    fn read_async(&mut self, buf: &mut [u8]) -> Box<AwaitBox<Result<usize>>>;
}

/// A trait for objects which are byte-oriented **asynchronous** sinks.
pub trait WriteAsync {
    fn write_async(&mut self, buf: &[u8]) -> Box<AwaitBox<Result<usize>>>;
    fn flush_async(&mut self) -> Box<AwaitBox<Result<()>>>;
}
