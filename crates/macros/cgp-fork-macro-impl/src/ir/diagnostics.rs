//! Error accumulation for the validate stage.

use syn::Error;

/// Collects every error a validation pass finds, so a user sees all of their mistakes in
/// one compile instead of one per fix.
///
/// Errors keep the order they were pushed in; [`Diagnostics::finish`] returns them as one
/// combined [`syn::Error`] whose first message is the first error pushed.
#[derive(Default)]
pub struct Diagnostics {
    error: Option<Error>,
}

impl Diagnostics {
    /// An empty accumulator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one error.
    pub fn push(&mut self, error: Error) {
        match &mut self.error {
            Some(existing) => existing.combine(error),
            None => self.error = Some(error),
        }
    }

    /// Records the error of `result`, if any, and returns its value otherwise.
    pub fn check<T>(&mut self, result: syn::Result<T>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                self.push(error);
                None
            }
        }
    }

    /// Whether no error was recorded.
    pub fn is_empty(&self) -> bool {
        self.error.is_none()
    }

    /// `Ok(())` when no error was recorded, otherwise every recorded error, combined.
    pub fn finish(self) -> syn::Result<()> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}
