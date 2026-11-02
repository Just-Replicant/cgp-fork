//! Test doubles for a component declared with `#[cgp_component(Name, mock)]`.
//!
//! The macro generates `MockName` and `NameMock`. A test context stores `NameMock`
//! in a field named `{name}_mock` (`greeter_mock` for `Greeter`) and derives
//! `HasField`. `MockName` reads that field.

/// [`alloc::boxed::Box`], so a generated mock can name it without the caller
/// linking `alloc` itself.
pub use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::cell::{Cell, RefCell};

/// Queued return values and a call count for one mocked method.
///
/// [`MockReturns::returning`] queues a value. [`MockReturns::next`] takes the next
/// one and counts the call. [`MockReturns::record`] counts a call that a closure
/// handled instead.
pub struct MockReturns<R> {
    calls: Cell<usize>,
    values: RefCell<VecDeque<R>>,
}

impl<R> MockReturns<R> {
    /// An empty queue and a zero call count.
    pub fn new() -> Self {
        Self {
            calls: Cell::new(0),
            values: RefCell::new(VecDeque::new()),
        }
    }

    /// Queue `value` for a later [`MockReturns::next`].
    pub fn returning(&self, value: R) {
        self.values.borrow_mut().push_back(value);
    }

    /// Calls so far, whether they were answered from the queue or by a closure.
    pub fn call_count(&self) -> usize {
        self.calls.get()
    }

    /// Count one call that did not take a queued value.
    pub fn record(&self) {
        self.calls.set(self.calls.get() + 1);
    }

    /// Take the next queued value and count the call.
    ///
    /// Panics when the queue is empty. `method` is the provider and method, so the
    /// message names what was not configured.
    pub fn next(&self, method: &'static str) -> R {
        self.record();
        self.values.borrow_mut().pop_front().unwrap_or_else(|| {
            panic!("no return value configured for {method}");
        })
    }
}

impl<R> Default for MockReturns<R> {
    fn default() -> Self {
        Self::new()
    }
}

/// Call `handler` without holding the slot's borrow.
///
/// The closure can call back into the mock. A recursive call of the same method
/// does not see this closure (it was taken out) and uses the queue instead. If the
/// closure installs a new closure, that replacement is kept; otherwise this one is
/// put back, including when `call` panics.
pub fn invoke_handler<F, R>(
    slot: &RefCell<Option<Box<F>>>,
    call: impl FnOnce(&mut F) -> R,
) -> Option<R>
where
    F: ?Sized,
{
    let handler = slot.borrow_mut().take()?;

    struct Restore<'a, F: ?Sized> {
        slot: &'a RefCell<Option<Box<F>>>,
        handler: Option<Box<F>>,
    }

    impl<F: ?Sized> Drop for Restore<'_, F> {
        fn drop(&mut self) {
            if let Some(handler) = self.handler.take() {
                let mut slot = self.slot.borrow_mut();

                if slot.is_none() {
                    *slot = Some(handler);
                }
            }
        }
    }

    let mut restore = Restore {
        slot,
        handler: Some(handler),
    };
    let result = call(restore.handler.as_mut().expect("handler was just taken"));
    Some(result)
}
