// Copied from `crates/main/cgp-fork/tests/runtime/has_runtime_mut.rs` with `cgp_fork` renamed to `cgp`.
//! `HasRuntimeMut` updates the same `runtime` field that `HasRuntime` reads.
//!
//! Both components are wired to `UseField<Symbol!("runtime")>`, so the mutable
//! getter does not require a field named `runtime_mut`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_getter.md and
//! cgp-knowledge-base-fork/cgp/reference/providers/use_field.md.

use cgp::extra::runtime::{
    HasRuntime, HasRuntimeMut, RuntimeGetterComponent, RuntimeMutGetterComponent,
    RuntimeTypeProviderComponent,
};
use cgp::prelude::*;

struct Clock {
    ticks: u32,
}

#[derive(HasField)]
struct App {
    runtime: Clock,
}

delegate_components! {
    App {
        RuntimeTypeProviderComponent: UseType<Clock>,
        RuntimeGetterComponent: UseField<Symbol!("runtime")>,
        RuntimeMutGetterComponent: UseField<Symbol!("runtime")>,
    }
}

#[test]
fn runtime_mut_updates_the_field() {
    let mut app = App {
        runtime: Clock { ticks: 1 },
    };

    assert_eq!(app.runtime().ticks, 1);

    app.runtime_mut().ticks += 1;

    assert_eq!(app.runtime().ticks, 2);
}
