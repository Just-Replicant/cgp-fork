//! Generic code raises errors through `CanRaiseError` and `CanWrapError`; each context
//! chooses the concrete error type. Here the same function runs against an `anyhow`
//! context and a boxed `std-error` context.
//!
//! Run with `cargo run -p cgp-fork --example error_backends --features anyhow,std-error`.

use cgp_fork::anyhow::{DisplayAnyhowError, RaiseAnyhowError, UseAnyhowError};
use cgp_fork::core::error::{
    ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp_fork::prelude::*;
use cgp_fork::std_error::{DisplayBoxedStdError, RaiseBoxedStdError, UseBoxedStdError};

/// Context-generic: knows nothing about anyhow or `Box<dyn Error>`.
fn parse_port<Context>(input: &str) -> Result<u16, Context::Error>
where
    Context: CanRaiseError<core::num::ParseIntError> + CanWrapError<String>,
{
    input
        .parse::<u16>()
        .map_err(Context::raise_error)
        .map_err(|error| Context::wrap_error(error, format!("invalid port {input:?}")))
}

pub struct AnyhowApp;

delegate_components! {
    AnyhowApp {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseAnyhowError,
        @ErrorRaiserComponent.core::num::ParseIntError: RaiseAnyhowError,
        @ErrorRaiserComponent.String: DisplayAnyhowError,
        ErrorWrapperComponent: DisplayAnyhowError,
    }
}

pub struct StdApp;

delegate_components! {
    StdApp {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseBoxedStdError,
        @ErrorRaiserComponent.core::num::ParseIntError: RaiseBoxedStdError,
        @ErrorRaiserComponent.String: DisplayBoxedStdError,
        ErrorWrapperComponent: DisplayBoxedStdError,
    }
}

fn main() {
    assert_eq!(parse_port::<AnyhowApp>("8080").ok(), Some(8080));

    let anyhow_error = parse_port::<AnyhowApp>("http").unwrap_err();
    println!("anyhow:    {anyhow_error:#}");
    assert!(format!("{anyhow_error:#}").starts_with("invalid port \"http\": "));

    let std_error = parse_port::<StdApp>("http").unwrap_err();
    println!("std-error: {std_error:#}");
    assert!(format!("{std_error:#}").starts_with("invalid port \"http\": "));
}
