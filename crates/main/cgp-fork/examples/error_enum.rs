//! An error enum whose `Display` text and source chain come from `#[cgp_auto_error]`.
//!
//! Run with `cargo run -p cgp-fork --example error_enum`.

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
}

fn main() {
    let error = AppError::Read {
        path: "note.txt",
        source: Disk,
    };
    assert_eq!(error.to_string(), "read note.txt");
    assert_eq!(Error::source(&error).unwrap().to_string(), "disk");
    println!("{error}: {}", Error::source(&error).unwrap());
}
