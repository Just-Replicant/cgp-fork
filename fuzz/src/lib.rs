//! The `cgp-fork-macro` implementation, compiled as an ordinary library so
//! libFuzzer can link it.
//!
//! The modules are the macro crate's own source files, included by path; only
//! the `#[proc_macro*]` wrappers in its `lib.rs` are left out. The entrypoint
//! list and seeds come from the in-tree robustness fuzzer
//! (`src/macro_tests/robustness_tests.rs`), so both fuzzers cover the same
//! macros. A new entrypoint added there is fuzzed here too.
#![allow(dead_code, unused_imports, clippy::all)]

#[path = "../../crates/macros/cgp-fork-macro/src/async_impl.rs"]
mod async_impl;
#[path = "../../crates/macros/cgp-fork-macro/src/extra_macro_core/mod.rs"]
mod extra_macro_core;
#[path = "../../crates/macros/cgp-fork-macro/src/extra_macro_lib/mod.rs"]
mod extra_macro_lib;
#[path = "../../crates/macros/cgp-fork-macro/src/macro_core/mod.rs"]
mod macro_core;
#[path = "../../crates/macros/cgp-fork-macro/src/macro_lib/mod.rs"]
mod macro_lib;
#[path = "../../crates/macros/cgp-fork-macro/src/macro_tests/robustness_tests.rs"]
mod robustness;

use std::str::FromStr;

use proc_macro2::TokenStream;
pub use robustness::{Entrypoint, SEEDS, entrypoints};

/// Run one fuzz input.
///
/// The first byte picks the entrypoint. The rest is UTF-8 text split at the
/// first `\u{1}` into the attribute and the item (no separator: empty
/// attribute). Text that does not lex as Rust tokens is skipped, as rustc would
/// reject it before the macro runs. Any panic is a finding: a macro must turn
/// bad input into a `syn::Error`.
pub fn run(data: &[u8]) {
    let Some((&selector, rest)) = data.split_first() else {
        return;
    };
    let Ok(text) = std::str::from_utf8(rest) else {
        return;
    };
    let (attr, body) = text.split_once('\u{1}').unwrap_or(("", text));
    let (Ok(attr), Ok(body)) = (TokenStream::from_str(attr), TokenStream::from_str(body)) else {
        return;
    };

    let entrypoints = entrypoints();
    let (_name, entrypoint) = entrypoints[usize::from(selector) % entrypoints.len()];
    let _ = entrypoint(attr, body);
}

/// Write the robustness seeds as a starting corpus: one file per seed and
/// entrypoint, in the format [`run`] reads.
pub fn write_seed_corpus(dir: &std::path::Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let count = entrypoints().len();
    let mut written = 0;
    for (seed_index, (attr, body)) in SEEDS.iter().enumerate() {
        for selector in 0..count {
            let mut bytes = vec![u8::try_from(selector).expect("fewer than 256 entrypoints")];
            if !attr.is_empty() {
                bytes.extend_from_slice(attr.as_bytes());
                bytes.push(1);
            }
            bytes.extend_from_slice(body.as_bytes());
            std::fs::write(
                dir.join(format!("seed-{seed_index:02}-{selector:02}")),
                bytes,
            )?;
            written += 1;
        }
    }
    Ok(written)
}
