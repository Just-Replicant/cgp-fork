use core::fmt::Debug;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, HasErrorType};
use crate::core::prelude::*;

/// Panics with the `Debug` formatting of the source instead of returning an error.
#[cgp_new_provider]
impl<Context, E> ErrorRaiser<Context, E> for PanicOnError
where
    Context: HasErrorType,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        panic!("{e:?}")
    }
}
