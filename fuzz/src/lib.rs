//! Fuzzing harness for the `cgp-fork` procedural macros.
//!
//! The macros are implemented in `cgp-fork-macro-impl`, an ordinary library, so
//! libFuzzer links it directly. The entrypoint list and seeds come from its
//! `registry` module, which the in-tree robustness fuzzer uses too, so both
//! fuzzers cover the same macros.

use std::str::FromStr;

pub use cgp_fork_macro_impl::registry::{Entrypoint, SEEDS, entrypoints};
use proc_macro2::TokenStream;

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
