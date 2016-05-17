//! `Await` implementations for _some_ `std::sync`` types

use Await;

use std::sync::{Mutex, MutexGuard, LockResult};

// Await `Mutex`
impl<'a, T: ?Sized> Await<LockResult<MutexGuard<'a, T>>> for &'a Mutex<T> {
    fn await(self) -> LockResult<MutexGuard<'a, T>> {
        self.lock()
    }
}

use std::sync::{Barrier, BarrierWaitResult};

// Await `Mutex`
impl<'a> Await<BarrierWaitResult> for &'a Barrier {
    fn await(self) -> BarrierWaitResult {
        self.wait()
    }
}


use std::sync::mpsc::{Receiver, RecvError};

// Await `Receiver`
impl<'a, T> Await<Result<T, RecvError>> for &'a Receiver<T> {
    fn await(self) -> Result<T, RecvError> {
        self.recv()
    }
}

