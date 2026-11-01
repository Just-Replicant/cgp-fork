//! One unit test per file. Each file is self-contained.

pub mod has_runtime_mut;
#[cfg(feature = "smol")]
pub mod smol_adapter;
#[cfg(feature = "tokio")]
pub mod tokio_adapter;
