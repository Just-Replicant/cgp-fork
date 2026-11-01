//! Fuzzing harness for the `cgp-fork` procedural macros.
//!
//! The macros are implemented in `cgp-fork-macro-impl`, an ordinary library, so
//! libFuzzer links it directly. The entrypoint list and seeds come from its
//! `registry` module, which the in-tree robustness fuzzer uses too, so both
//! fuzzers cover the same macros.

use std::path::Path;
use std::str::FromStr;

pub use cgp_fork_macro_impl::registry::{Entrypoint, SEEDS, entrypoints, fuzz_target_name};
use proc_macro2::TokenStream;

/// Split a fuzz input into the attribute and the item.
///
/// The input is UTF-8 text, split at the first `\u{1}` into the attribute and the
/// item (no separator: empty attribute). Text that does not lex as Rust tokens
/// yields `None`, as rustc would reject it before the macro runs.
fn split_input(data: &[u8]) -> Option<(TokenStream, TokenStream)> {
    let text = std::str::from_utf8(data).ok()?;
    let (attr, body) = text.split_once('\u{1}').unwrap_or(("", text));
    Some((
        TokenStream::from_str(attr).ok()?,
        TokenStream::from_str(body).ok()?,
    ))
}

/// Run one input of the all-entrypoints target.
///
/// The first byte picks the entrypoint (modulo their number); the rest is read
/// as [`split_input`] describes. Any panic is a finding: a macro must turn bad
/// input into a `syn::Error`.
pub fn run(data: &[u8]) {
    let Some((&selector, rest)) = data.split_first() else {
        return;
    };
    let Some((attr, body)) = split_input(rest) else {
        return;
    };
    let entrypoints = entrypoints();
    let (_name, entrypoint) = entrypoints[usize::from(selector) % entrypoints.len()];
    let _ = entrypoint(attr, body);
}

/// Run one input of a single-entrypoint target, named as in the registry.
///
/// # Panics
///
/// When `name` is not a registered entrypoint, which is a bug in the generated
/// target, not a finding.
pub fn run_entrypoint(name: &str, data: &[u8]) {
    let (_name, entrypoint) = entrypoints()
        .into_iter()
        .find(|(entry, _)| *entry == name)
        .unwrap_or_else(|| panic!("`{name}` is not a registered entrypoint"));
    let Some((attr, body)) = split_input(data) else {
        return;
    };
    let _ = entrypoint(attr, body);
}

/// Encode one seed as a single-entrypoint input: the attribute, `\u{1}`, the item.
fn encode_seed(attr: &str, body: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    if !attr.is_empty() {
        bytes.extend_from_slice(attr.as_bytes());
        bytes.push(1);
    }
    bytes.extend_from_slice(body.as_bytes());
    bytes
}

/// Write the robustness seeds as starting corpora under `root`:
/// `root/entrypoints/` for the all-entrypoints target (one file per seed and
/// entrypoint, with the selector byte) and `root/<target>/` for each
/// single-entrypoint target. Returns the number of files written.
pub fn write_seed_corpus(root: &Path) -> std::io::Result<usize> {
    let entrypoints = entrypoints();
    let mut written = 0;

    let all = root.join("entrypoints");
    std::fs::create_dir_all(&all)?;
    for (seed_index, (attr, body)) in SEEDS.iter().enumerate() {
        for selector in 0..entrypoints.len() {
            let mut bytes = vec![u8::try_from(selector).expect("fewer than 256 entrypoints")];
            bytes.extend(encode_seed(attr, body));
            std::fs::write(
                all.join(format!("seed-{seed_index:02}-{selector:02}")),
                bytes,
            )?;
            written += 1;
        }
    }

    for (name, _) in &entrypoints {
        let dir = root.join(fuzz_target_name(name));
        std::fs::create_dir_all(&dir)?;
        for (seed_index, (attr, body)) in SEEDS.iter().enumerate() {
            std::fs::write(
                dir.join(format!("seed-{seed_index:02}")),
                encode_seed(attr, body),
            )?;
            written += 1;
        }
    }
    Ok(written)
}
