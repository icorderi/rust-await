//! Helper functions
//!
//! Currently harcoded to work with `Thread`'s
//!
//! **Example: This does not need to be on this crate**

use std::thread::{self, JoinHandle, Result};
use std::sync::mpsc::channel;

use {Await, AwaitBox};

/// Starts a new `Thread` and executes the `Await` there.
/// This method returns immediatly.
pub fn spawn<T: Send + 'static, A: Await<T> + Send + 'static>(a: A) -> JoinHandle<T> {
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

/// Forks all the `Await`'s in parallel.
///
/// The returning `Await` will complete when all input `Await`'s complete.
///
/// **XXX:** this should queue a new work item with the _execution engine_
pub fn parallel<T: Send + 'static>(xs: Vec<Box<AwaitBox<T> + Send>>) -> Box<AwaitBox<Vec<Result<T>>>> {
    await_all(xs.into_iter().map(|x| spawn(x)).collect())
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

/// Returns an `Await` that will _await_ for **any** of the inputs to finish.
/// This method returns immediatly.
///
/// **TODO:** once **impl Trait** lands we can change this to `.. -> impl Await<T>`
pub fn any<T: Send + 'static>(xs: Vec<Box<AwaitBox<T> + Send>>) -> Box<AwaitBox<T>> {
    let (tx, rx) = channel();
    for x in xs {
        let tx = tx.clone();
        spawn(move || {
            let t = x.await();
            tx.send(t)
        });
    }
    Box::new(move || rx.await().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_join() {
        let aw = parallel(vec![Box::new(||7), Box::new(||8), Box::new(||9)]);
        let xs: Vec<usize> = run_synchronously(aw)
                                .into_iter()
                                .map(|x| x.unwrap())
                                .collect();
        assert_eq!(vec![7,8,9], xs);
    }

    #[test]
    fn fork_any() {
        let aw = any(vec![Box::new(||7), Box::new(||8), Box::new(||9)]);
        let x: usize = run_synchronously(aw);
        assert!(vec![7,8,9].contains(&x));
    }

}
