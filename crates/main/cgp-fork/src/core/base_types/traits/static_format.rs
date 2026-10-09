use core::fmt::{self, Formatter};

use crate::core::base_types::types::{Chars, Nil};

/// Formats a type-level string without a value.
///
/// [`Symbol`](crate::core::base_types::types::Symbol) and [`Chars`](crate::core::base_types::types::Chars)
/// implement this so a type-level string can be written with `{}`.
pub trait StaticFormat {
    /// Writes the type-level characters into `f`.
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}

impl<const CHAR: char, Tail> StaticFormat for Chars<CHAR, Tail>
where
    Tail: StaticFormat,
{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{CHAR}")?;
        Tail::fmt(f)
    }
}

impl StaticFormat for Nil {
    fn fmt(_f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
