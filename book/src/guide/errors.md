# Errors

A context's error is an associated type on `HasErrorType`. `CanRaiseError<Source>`
turns a concrete error into that type. `CanWrapError<Detail>` adds context.
`RaiseFrom` uses `From`. `#[cgp_auto_error]` on an inherent impl builds the three
providers from `type Error`, `raise_error`, and `wrap_error`.

On an enum, the same attribute derives `Display`, `core::error::Error`, and
`From`.

- `#[derive(Debug)]` is required, because `Error` requires `Debug`.
- `#[error("...")]` is the `Display` text. `{field}` and `{field:?}` (or `{0}` on
  a tuple variant) read the fields. `{{` and `}}` are literal braces.
- `#[error(transparent)]` forwards `Display` and `source` to the single field.
- `#[source]` is the field `Error::source` returns. It must be `'static`.
- `#[from]` on a one-field variant implies `#[source]` and implements `From`.

The attribute does not build the error providers. Wire `RaiseFrom` when the
context's error is this enum, or keep using the impl form for a custom raiser.

```rust
use core::error::Error;
use core::fmt::{self, Display};

use cgp_fork::prelude::*;

#[derive(Debug)]
struct Disk;

impl Display for Disk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("disk")
    }
}

impl Error for Disk {}

#[derive(Debug)]
#[cgp_auto_error]
enum AppError {
    #[error("read {path}")]
    Read {
        path: &'static str,
        #[source]
        source: Disk,
    },
    #[error("missing {0}")]
    Missing(&'static str),
    #[error(transparent)]
    Other(#[from] Disk),
}

fn main() {
    let error = AppError::Read {
        path: "note.txt",
        source: Disk,
    };
    assert_eq!(error.to_string(), "read note.txt");
    assert_eq!(Error::source(&error).unwrap().to_string(), "disk");

    let missing = AppError::Missing("id");
    assert_eq!(missing.to_string(), "missing id");
    assert!(Error::source(&missing).is_none());

    let wrapped = AppError::from(Disk);
    assert_eq!(wrapped.to_string(), "disk");
}
```

`anyhow`, `eyre`, and `std-error` are features that pick a ready-made error type.
They are documented on the [features](../reference/features.md) page.
