use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use crate::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorWrapper, ErrorWrapperComponent};
use crate::core::prelude::*;

/// Raises and wraps by formatting with `{:?}`, then raising the resulting `String`.
///
/// The original value is not kept. The context must be able to raise and wrap a `String`.
pub struct DebugError;

#[cgp_provider]
impl<Context, E> ErrorRaiser<Context, E> for DebugError
where
    Context: CanRaiseError<String>,
    E: Debug,
{
    fn raise_error(e: E) -> Context::Error {
        Context::raise_error(format!("{e:?}"))
    }
}

#[cgp_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DebugError
where
    Context: CanWrapError<String>,
    Detail: Debug,
{
    fn wrap_error(error: Context::Error, detail: Detail) -> Context::Error {
        Context::wrap_error(error, format!("{detail:?}"))
    }
}
