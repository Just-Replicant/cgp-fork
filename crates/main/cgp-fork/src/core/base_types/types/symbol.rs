use core::fmt::Display;
use core::marker::PhantomData;

/// A type-level string: `LEN` is the byte length and `Chars` is the [`Chars`](super::Chars) chain.
///
/// [`Symbol!`](crate::core::macros::Symbol) builds this from a string literal. The byte length is stored
/// beside the characters so a symbol can be named without walking the chain.
pub struct Symbol<const LEN: usize, Chars>(pub PhantomData<Chars>);

use crate::core::base_types::traits::StaticFormat;

impl<const LEN: usize, Chars> Default for Symbol<LEN, Chars> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<const LEN: usize, Chars> StaticFormat for Symbol<LEN, Chars>
where
    Chars: StaticFormat,
{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Chars::fmt(f)
    }
}

impl<const LEN: usize, Chars> Display for Symbol<LEN, Chars>
where
    Self: StaticFormat,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as StaticFormat>::fmt(f)
    }
}
