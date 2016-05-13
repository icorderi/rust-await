//! Helper functions
//!
//! **Example: This does not need to be on this crate**

use std::thread::{self, JoinHandle};

use {Await, AwaitBox};

/// Starts a new `Thread` and executes the `Await` there.
/// This method returns immediatly.
pub fn spawn<'a, T: Send + 'static, A: Await<T> + Send + 'static>(a: A) -> JoinHandle<T> {
    thread::spawn(|| a.await())
}

/// Runs the `Await` on the current thread and **wont** return until the result is ready.
///
/// This does **not** mean that the current thread will _block_ or even _yield_.
/// Any _blocking_, _pause/resume_, or _yielding_ will depend on the `Await` task being
/// executed and the _execution engine_.
pub fn run_synchronously<T, A: Await<T>>(a: A) -> T {
    a.await()
}

/// Returns an `Await` that will _await_ for **all** inputs to finish and produce a `Vec`
/// with the results.
/// This method returns immediatly.
///
/// **TODO:** once **impl Trait** lands we can change this to `.. -> impl Await<Vec<T>>`
pub fn await_all<T, A: Await<T> + 'static>(xs: Vec<A>) -> Box<AwaitBox<Vec<T>>> {
    let f = || xs.into_iter().map(|x| x.await()).collect();
    Box::new(f)
}

/// Returns an `Await` that will _await_ for **all** inputs to finish and produce a `Vec`
/// with the results.
/// This method returns immediatly.
///
/// **TODO:** once **impl Trait** lands we can change this to `.. -> impl Await<Vec<T>>`
pub fn await_all_box<T: 'static>(xs: Vec<Box<AwaitBox<T>>>) -> Box<AwaitBox<Vec<T>>> {
    let f = || xs.into_iter().map(|x| x.await()).collect();
    Box::new(f)
}
