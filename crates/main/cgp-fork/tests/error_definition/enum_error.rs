//! `#[cgp_auto_error]` on an enum implements `Display` and `core::error::Error`.
//!
//! `#[source]` (and `#[from]`, which implies it) is the source chain. `#[from]`
//! on a one-field variant also implements `From`. A `{field:?}` placeholder
//! formats with `Debug`.

use core::error::Error;
use core::fmt::{self, Display};

use cgp_fork::prelude::*;

#[derive(Debug)]
struct Detail(String);

impl Display for Detail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for Detail {}

#[derive(Debug)]
#[cgp_auto_error]
enum AppError<E> {
    #[error("read {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: E,
    },
    #[error("missing {name:?}")]
    Missing { name: String },
    #[error("pair {0} {1}")]
    Pair(u8, u8),
    #[error(transparent)]
    Other(#[from] E),
    #[error("braces {{ok}}")]
    Braces,
}

#[test]
fn formats_fields_and_keeps_the_source() {
    let error = AppError::<Detail>::Read {
        path: "note.txt".to_owned(),
        source: Detail("disk".to_owned()),
    };
    assert_eq!(error.to_string(), "read note.txt: disk");
    let source = Error::source(&error).expect("source");
    assert_eq!(source.to_string(), "disk");

    let missing = AppError::<Detail>::Missing {
        name: "id".to_owned(),
    };
    assert_eq!(missing.to_string(), "missing \"id\"");
    assert!(Error::source(&missing).is_none());

    assert_eq!(AppError::<Detail>::Pair(1, 2).to_string(), "pair 1 2");
    assert_eq!(AppError::<Detail>::Braces.to_string(), "braces {ok}");
}

#[test]
fn from_uses_the_transparent_variant() {
    let error = AppError::from(Detail("io".to_owned()));
    assert_eq!(error.to_string(), "io");
    assert_eq!(Error::source(&error).expect("source").to_string(), "io");
}
