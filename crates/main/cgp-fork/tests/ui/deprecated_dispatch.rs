//! The 0.10 dispatchers are deprecated; the warning names the unified `MatchWith`.
#![deny(deprecated)]

use cgp_fork::extra::dispatch::MatchWithHandlers;
use cgp_fork::prelude::*;

pub type Dispatch = MatchWithHandlers<Product![]>;

fn main() {}
