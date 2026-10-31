use core::marker::PhantomData;

/// A `Code` of `()` for a handler that does not need to tell computations apart.
#[allow(non_upper_case_globals)]
pub const NoCode: PhantomData<()> = PhantomData;

/// Dispatches a handler on its input type through the `Components` table.
///
/// `DelegateComponent` is implemented for the input type (or a pair of input and args). The
/// delegate is the provider that handles that input.
pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
