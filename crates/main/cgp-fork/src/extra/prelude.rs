//! Handlers, matchers, and the macros that turn a function into a provider.
//!
//! The handler and dispatch names are the 0.10 family and need the `compat-0.10` feature; the
//! unified family is in [`crate::handler::prelude`].

pub use cgp_fork_macro::{cgp_auto_dispatch, cgp_auto_log, cgp_handler};
#[cfg(feature = "compat-0.10")]
#[allow(deprecated)]
pub use cgp_fork_macro::{cgp_computer, cgp_producer};

#[cfg(feature = "compat-0.10")]
#[allow(deprecated)]
pub use crate::extra::dispatch::{
    MatchFirstWithValueHandlers, MatchFirstWithValueHandlersMut, MatchFirstWithValueHandlersRef,
    MatchWithValueHandlers, MatchWithValueHandlersMut, MatchWithValueHandlersRef,
};
#[cfg(feature = "compat-0.10")]
#[allow(deprecated)]
pub use crate::extra::handler::{
    AsyncComputer, AsyncComputerComponent, AsyncComputerRef, AsyncComputerRefComponent, Computer,
    ComputerComponent, ComputerRefComponent, Handler, HandlerComponent, HandlerRefComponent,
    Producer, ProducerComponent, PromoteAsyncComputer, PromoteComputer, PromoteHandler,
    PromoteProducer, PromoteTryComputer, TryComputer, TryComputerComponent,
    TryComputerRefComponent,
};
pub use crate::extra::log::{CanLog, Logger, LoggerComponent};
