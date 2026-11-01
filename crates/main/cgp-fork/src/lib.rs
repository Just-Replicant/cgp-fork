#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate self as cgp_fork;

/// Components, providers, and the type-level lists they are wired through.
pub mod core;
/// Handlers, dispatch, logging, and the other providers built on [`core`].
pub mod extra;
// `handler` and `compat` are documented by their own `//!` comments, whose links resolve in
// the module.
pub mod handler;

#[cfg(feature = "compat-0.10")]
pub mod compat;

/// The names most programs import: `use cgp_fork::prelude::*;`.
pub mod prelude;

/// Names the macro expansion refers to. A superset of [`prelude`], so generated
/// paths such as `::cgp_fork::macro_prelude::ErrorRaiser` resolve without exporting
/// the provider traits from the user prelude.
pub mod macro_prelude {
    pub use crate::__cgp_compat_0_10;
    #[cfg(feature = "compat-0.10")]
    pub use crate::compat::Compat;
    pub use crate::core::error::{
        ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
        ErrorWrapper, ErrorWrapperComponent,
    };
    // The access modes the derived extractors are generic over.
    pub use crate::core::field::types::{Mut, Owned, Ref};
    /// The unified handler family, for `#[cgp_handler]`, `#[derive_handler]` and
    /// `#[cgp_auto_dispatch]`. A submodule, because the 0.10 names (`Handler`,
    /// `HandlerComponent`, `Promote`) are in this one; and not a renamed re-export, so
    /// diagnostics keep naming the items by their public paths (`cgp_fork::handler::Handler`).
    #[doc(hidden)]
    pub mod unified {
        pub use crate::handler::{
            All, AsyncHandler, AsyncHandlerComponent, First, Handler, HandlerComponent,
            MatchValues, Promote,
        };
    }
    pub use crate::prelude::*;

    /// Named by `#[cgp_component]` at a `#[derive_promote]` attribute, so its user is
    /// warned with the replacement.
    #[doc(hidden)]
    #[deprecated(
        since = "0.11.0",
        note = "`#[derive_promote(Name)]` forwards to a 0.10 `Computer`; use `#[derive_handler(Name)]`, which forwards to a unified `Handler`"
    )]
    pub const DERIVE_PROMOTE: () = ();
}

/// Expands to its input when this crate has the `compat-0.10` feature, and to nothing
/// otherwise. Generated code wraps its 0.10 wiring in it, because the feature that decides
/// is this crate's, not the caller's.
#[doc(hidden)]
#[cfg(feature = "compat-0.10")]
#[macro_export]
macro_rules! __cgp_compat_0_10 {
    ($($item:tt)*) => { $($item)* };
}

/// Expands to its input when this crate has the `compat-0.10` feature, and to nothing
/// otherwise.
#[doc(hidden)]
#[cfg(not(feature = "compat-0.10"))]
#[macro_export]
macro_rules! __cgp_compat_0_10 {
    ($($item:tt)*) => {};
}

#[cfg(feature = "anyhow")]
#[doc = include_str!("anyhow/README.md")]
pub mod anyhow;

#[cfg(feature = "eyre")]
#[doc = include_str!("eyre/README.md")]
pub mod eyre;

#[cfg(feature = "std-error")]
#[doc = include_str!("std_error/README.md")]
pub mod std_error;
