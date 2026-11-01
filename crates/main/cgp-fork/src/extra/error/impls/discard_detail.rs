use crate::core::error::{ErrorWrapper, ErrorWrapperComponent, HasErrorType};
use crate::core::prelude::*;

/// Returns the error unchanged and drops the detail.
#[cgp_new_provider]
impl<Context, Detail> ErrorWrapper<Context, Detail> for DiscardDetail
where
    Context: HasErrorType,
{
    fn wrap_error(error: Context::Error, _detail: Detail) -> Context::Error {
        error
    }
}
