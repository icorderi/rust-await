//! `Await` implementations for _some_ `std::net`` types

use std::io::{Result, Read, Write};
use std::net::TcpStream;

use io::{ReadAsync, WriteAsync};

use {AwaitBox, AwaitValue};

impl ReadAsync for TcpStream {
    fn read_async(&mut self, buf: &mut [u8]) -> Box<AwaitBox<Result<usize>>> {
        // Just call `read` for now
        let r = self.read(buf);
        Box::new(AwaitValue(r))
    }
}

impl WriteAsync for TcpStream {
    fn write_async(&mut self, buf: &[u8]) -> Box<AwaitBox<Result<usize>>> {
        // Just call `write` for now
        let r = self.write(buf);
        Box::new(AwaitValue(r))
    }

    fn flush_async(&mut self) -> Box<AwaitBox<Result<()>>> {
        // Just call `flush` for now
        let r = self.flush();
        Box::new(AwaitValue(r))
    }
}
