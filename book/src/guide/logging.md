# Logging

`#[cgp_auto_log]` turns each method of a logger trait into a `CanLog` call. The
detail struct is named after the method (`log_hello` becomes `__LogHello`).

`CanLog<Detail>` records a value. `TracingLogger` formats it with `Display` and
`tracing::info`. `TracingDebugLogger` formats it with `Debug`. Both need the
`log-tracing` feature. `DefmtLogger` needs `log-defmt`.

Without a tracing subscriber the event is discarded and the call still returns.
`CanLog<Detail>` is generic, so the automatic `()` check is skipped. Keep
`#[unchecked]` and name the detail in `check_components!`.

```rust
use std::fmt::{self, Display};

use cgp_fork::extra::log::{TracingDebugLogger, TracingLogger};
use cgp_fork::prelude::*;

pub struct Started {
    pub name: &'static str,
}

impl Display for Started {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "started {}", self.name)
    }
}

pub struct App;

delegate_components! {
    #[unchecked]
    App {
        LoggerComponent: TracingLogger,
    }
}

check_components! {
    App {
        LoggerComponent: Started,
    }
}

#[derive(Debug)]
pub struct Tick {
    pub n: u32,
}

pub struct DebugApp;

delegate_components! {
    #[unchecked]
    DebugApp {
        LoggerComponent: TracingDebugLogger,
    }
}

check_components! {
    DebugApp {
        LoggerComponent: Tick,
    }
}

fn main() {
    App.log(Started { name: "Ferris" });
    DebugApp.log(Tick { n: 1 });
}
```
