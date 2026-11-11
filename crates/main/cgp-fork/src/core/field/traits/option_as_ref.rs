use core::convert::AsRef;

/// Borrows the value inside an [`Option`] as `&T`.
///
/// A shared `#[implicit]` argument of type `Option<&str>` generates this bound, so
/// the field may be any `Option<U>` whose `U` implements [`AsRef<str>`](AsRef): a
/// `String`, a `Cow<str>`, or a `Box<str>`. The read needs no allocator.
///
/// A mutable `Option<&mut str>` does not use this trait. It still reads an
/// `Option<String>` field with `Option::as_deref_mut`.
pub trait OptionAsRef<T: ?Sized> {
    /// Borrows the contained value, or `None` when the option is empty.
    fn option_as_ref(&self) -> Option<&T>;
}

impl<T: ?Sized, U> OptionAsRef<T> for Option<U>
where
    U: AsRef<T>,
{
    fn option_as_ref(&self) -> Option<&T> {
        self.as_ref().map(AsRef::as_ref)
    }
}
