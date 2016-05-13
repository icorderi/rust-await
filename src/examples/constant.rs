//! When they need an `Await<T>` and there is no need to wait
//!
//! **TODO: Should this be part of the `await` module?**

use Await;

/// No waiting, we already have the value
pub struct AwaitValue<T>(pub T);

impl<T> Await<T> for AwaitValue<T> {
    fn await(self) -> T {
        self.0
    }
}
