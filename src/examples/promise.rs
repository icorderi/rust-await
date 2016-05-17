//! _"Crate"_ for rerpresenting the _async_ abstraction of **promises / futures**
//!
//! - A `Future` represents the potential of a value in the future
//! - A `Promise` is used to complete (a.k.a fulfill) the `Future`.
//!
//! One can say that a `Future<T>` is in essence an `Await<T>`, and they wouldn't be wrong.
//!
//! **Example: This does not need to be on this crate**

use std::sync::mpsc::{SyncSender, sync_channel, SendError};
use std::mem;

use {Await, AwaitBox};

/// At some point in time, it will hold a value `T`
pub struct Future<T> {
    inner: FutureState<T>,
}

enum FutureState<T> {
    Value(T),
    /// **TODO**: should this be `Box` or make a type param `A: Await<T>`
    Deffered(Box<AwaitBox<T> + Send>),
    Evaluating,
}

/// Used to fulfill a promise (in other words, complete the `Future`)
///
/// This can only be owned by **one** thread, but it can be `clone`'d to _send_ to other threads.
///
/// **XXX**: this could use `mio` for non-"thread"-blocking
///
/// **TODO**: should `Promise` be a _trait_?
#[derive(Clone)]
pub struct Promise<T> {
    tx: SyncSender<T>,
}

/// Signals that the `Promise` dropped before being fullfilled. _Liar..._
#[derive(Debug)]
pub struct PromiseDroppedError;

impl<T: 'static + Send> Future<Result<T, PromiseDroppedError>> {
    /// Create a promise from anything that can be _await'ed_
    ///
    /// **XXX**: this could use `mio` for non-"thread"-blocking
    pub fn new() -> (Future<Result<T, PromiseDroppedError>>, Promise<T>) {
        let (tx, rx) = sync_channel(1);
        let f = Promise { tx: tx };
        let p = Future {
            inner: FutureState::Deffered(Box::new(move || rx.recv().map_err(|_| PromiseDroppedError))),
        };
        (p, f)
    }
}

impl<T> Future<T> {
    /// Create a `Future` that holds a _constant_ value... **it's a present!** _(no pun intended...)_
    pub fn constant(value: T) -> Self {
        Future { inner: FutureState::Value(value) }
    }

    /// Create a `Future` from an `Await`
    pub fn deferred<A: Await<T> + 'static + Send>(x: A) -> Self {
        Future { inner: FutureState::Deffered(Box::new(x)) }
    }

    /// Returns a _ref_ to the value `T` if present
    pub fn poll(&self) -> Option<&T> {
        match self.inner {
            FutureState::Value(ref v) => Some(v),
            FutureState::Deffered(_) => None,
            FutureState::Evaluating => unreachable!(),
        }
    }

    /// Returns a _ref_ to the value `T`, _awaits_ for the value if it's not available
    ///
    /// **XXX**: should we use inner mutability to get rid of `&mut`?
    pub fn value(&mut self) -> &T {
        let x = mem::replace(&mut self.inner, FutureState::Evaluating);
        let v = match x {
            FutureState::Value(v) => v,
            FutureState::Deffered(a) => a.await(),
            FutureState::Evaluating => unreachable!(),
        };
        mem::replace(&mut self.inner, FutureState::Value(v));
        match self.inner {
            FutureState::Value(ref v) => v,
            _ => unreachable!(),
        }
    }
}

impl<T> Promise<T> {
    /// Fullfill a promise
    ///
    /// A promise can only be used **once**
    ///
    /// # Returns
    /// `None` if the promise was delivered.
    /// If the `Future` is dropped before the promise is fulfilled, this will return `Some(value)`
    pub fn set(self, value: T) -> Option<T> {
        match self.tx.send(value) {
            Ok(_) => None,
            Err(SendError(v)) => Some(v),
        }
    }
}

impl<T, E> Promise<Result<T, E>> {
    /// Deliver a successful result
    ///
    /// Convenience wrapper around `self.set` for the `Ok()` case
    #[inline]
    pub fn success(self, value: T) -> Option<T> {
        match self.set(Ok(value)) {
            Some(Ok(v)) => Some(v),
            Some(_) => unreachable!(),
            None => None,
        }
    }

    /// Deliver a failure
    ///
    /// Convenience wrapper around `self.set` for the `Err()` case
    #[inline]
    pub fn fail_with(self, err: E) -> Option<E> {
        match self.set(Err(err)) {
            Some(Err(err)) => Some(err),
            Some(_) => unreachable!(),
            None => None,
        }
    }
}

impl<T> Await<T> for Future<T> {
    fn await(self) -> T {
        match self.inner {
            FutureState::Value(v) => v,
            FutureState::Deffered(a) => a.await(),
            FutureState::Evaluating => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Await;

    #[test]
    fn present() {
        let x = Future::constant(8);
        assert_eq!(8, x.await());
    }

    #[test]
    fn deferred() {
        let x = Future::deferred(Future::constant(8));
        assert_eq!(8, x.await());
    }

    #[test]
    fn promise() {
        use std::thread;
        use std::time::Duration;

        let (future, promise) = Future::new();

        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            promise.set(8);
        });

        assert_eq!(8, future.await().unwrap());
    }

    #[test]
    fn threads() {
        use std::thread;
        use std::time::Duration;

        let (future, promise) = Future::new();

        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            promise.set(8);
        });

        let t = thread::spawn(|| future.await().unwrap());

        assert_eq!(8, t.join().unwrap());
    }

    #[test]
    fn racing() {
        use std::thread;
        use std::time::Duration;

        let (future, promise) = Future::new();

        let p1 = promise.clone();
        let p2 = promise.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            p1.set(8);
        });

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            p2.set(8);
        });

        let t = thread::spawn(|| future.await().unwrap());

        assert_eq!(8, t.join().unwrap());
    }
}
