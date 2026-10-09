use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use crate::core::prelude::*;

/// Returns `E` as the abstract error. The context's error type must already be `E`.
#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for ReturnError
where
    Context: HasErrorType<Error = E>,
{
    fn raise_error(e: E) -> E {
        e
    }
}
