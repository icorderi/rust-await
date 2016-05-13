//! Core primitives for building asynchronous stuff
//!
//! # This is [**Proposal A**](https://github.com/icorderi/rust-await/tree/proposal/a)
//!
//! This prposal is based on the idea of adding a single `Await` trait to the core.
//! The `Await` trait would provide a common abstraction over things that we could **resume** from.
//!
//! In the following example (with compiler support):
//!
//! ```ignore
//! pub **async** fn my_code() {
//!     // code block A
//!     let x = **await** some_func()
//!     // code block B
//! }
//! ```
//!
//! the expectation is that _"code block B"_ will be _resumed_ whenever the `await`'ing of `some_func` returns.
//!
//! For now, without compiler support, it would look like:
//!
//! ```ignore
//! pub fn my_code() {
//!     // code block A
//!     let x = some_func().await()
//!     // code block B
//! }
//! ```

pub mod examples;

/// Represents then notion of something that we **could** have to _await_ for.
///
/// The keyword **await** _could_ be added to the language.
pub trait Await<T> {
    /// It will return a value when it's ready
    fn await(self) -> T;
}

/// `AwaitBox` is a version of the `Await` intended for use with _boxed_
/// objects.
///
/// The idea is that where one would normally store a
/// `Box<Await<T>>` in a data structure, you should use
/// `Box<AwaitBox<T>>`. The two traits behave essentially the same, except
/// that a `AwaitBox` can **only** be called if it is _boxed_.
///
/// > Note that `AwaitBox` may be deprecated in the future if `Box<Await<T>>`
/// become directly usable.
pub trait AwaitBox<T> {
    fn await_box(self: Box<Self>) -> T;
}

impl<T, A> AwaitBox<T> for A
    where A: Await<T>
{
    fn await_box(self: Box<Self>) -> T {
        self.await()
    }
}

impl<'a, T> Await<T> for Box<AwaitBox<T> + 'a> {
    fn await(self) -> T {
        self.await_box()
    }
}

// ============================================================================
//      `Await` for std types
// ============================================================================

// // `Await` on `Fn`
// impl<T, F: Fn() -> T> Await<T> for F {
//     fn await(self) -> T {
//         self()
//     }
// }

// `Await` on `FnOnce`
impl<T, F: FnOnce() -> T> Await<T> for F {
    fn await(self) -> T {
        self()
    }
}

use std::thread::JoinHandle;
use std::any::Any;

// Await on threads
impl<T> Await<Result<T, Box<Any + Send + 'static>>> for JoinHandle<T> {
    fn await(self) -> Result<T, Box<Any + Send + 'static>> {
        self.join()
    }
}

use std::sync::mpsc::{Receiver, RecvError};

// Await `Receiver`
impl<T> Await<Result<T, RecvError>> for Receiver<T> {
    fn await(self) -> Result<T, RecvError> {
        self.recv()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use Await;

    #[test]
    fn await_fn() {
        let x = || 8;
        assert_eq!(8, x.await());
    }

    #[test]
    fn await_thread() {
        let t = thread::spawn(|| 8);
        assert_eq!(8, t.await().unwrap());
    }

    #[test]
    fn await_box() {
        let x = Box::new(|| 8);
        assert_eq!(8, x.await());
    }
}
