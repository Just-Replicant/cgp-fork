use core::marker::PhantomData;

/// A `Code` of `()` for a handler that does not need to tell computations apart.
#[allow(non_upper_case_globals)]
pub const NoCode: PhantomData<()> = PhantomData;

pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
