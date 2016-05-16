//! `Await` implementations for _some_ `std::thread`` types

use Await;

use std::thread::JoinHandle;
use std::any::Any;

// Await on threads
impl<T> Await<Result<T, Box<Any + Send + 'static>>> for JoinHandle<T> {
    fn await(self) -> Result<T, Box<Any + Send + 'static>> {
        self.join()
    }
}
