//! _"Crate"_ for rerpresenting the _async_ abstraction of **promises / futures**
//!
//! - A `Future` represents the potential of a value in the future
//! - A `Promise` is used to complete (a.k.a fulfill) the `Future`.
//!
//! One can say that a `Future<T>` is in essence an `Await<T>`, and they wouldn't be wrong.
//!
//! **Example: This does not need to be on this crate**

use std::sync::mpsc::{SyncSender, sync_channel};
use std::mem;

use {Await, AwaitBox};

/// At some point in time, it will hold a value `T`
pub struct Future<T> {
    inner: FutureState<T>,
}

enum FutureState<T> {
    Value(T),
    /// **TODO**: should this be `Box` or make a type param `A: Await<T>`
    Deffered(Box<AwaitBox<T>>),
    Evaluating,
}

/// Used to fulfill a promise
///
/// **XXX**: this could use `mio` for non-"thread"-blocking
/// **TODO**: should `Promise` be a _trait_?
pub struct Promise<T> {
    tx: SyncSender<T>,
}

/// Signals that the `Promise` dropped before being fullfilled. _Liar..._
#[derive(Debug)]
pub struct PromiseDroppedError;

impl<T: 'static> Future<Result<T, PromiseDroppedError>> {
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
    pub fn deferred<A: Await<T> + 'static>(x: A) -> Self {
        Future { inner: FutureState::Deffered(Box::new(x)) }
    }

    /// Returns a ref to the value `T`, _awaits_ for the value if it's not available
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
    /// _true_ if promsie was fulfilled,
    /// if the `Future` is dropped before the promise is fulfilled, this will return _false_
    pub fn set(self, value: T) -> bool {
        match self.tx.send(value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl<T,E> Promise<Result<T,E>> {
    /// Deliver a successful result
    ///
    /// Convenience wrapper around `self.set` for the `Ok()` case
    #[inline]
    pub fn success(self, value: T) -> bool {
        self.set(Ok(value))
    }

    /// Deliver a failure
    ///
    /// Convenience wrapper around `self.set` for the `Err()` case
    #[inline]
    pub fn fail_with(self, err: E) -> bool {
        self.set(Err(err))
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
    fn example_1() {
        let x = Future::constant(8);
        assert_eq!(8, x.await());
    }

    #[test]
    fn example_2() {
        let x = Future::deferred(Future::constant(8));
        assert_eq!(8, x.await());
    }

    #[test]
    fn example_3() {
        use std::thread;
        use std::time::Duration;

        let (future,promise) = Future::new();

        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            promise.set(8); });

        assert_eq!(8, future.await().unwrap());
    }
}
