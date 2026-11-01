//! Extracts the `rust,ignore` examples from the macro crate's docs so they can
//! be compiled here, against the `cgp-fork` facade.
//!
//! `cgp-fork-macro` is a proc-macro crate: it cannot dev-depend on `cgp-fork`
//! (that would be a dependency cycle), so its own doctests must stay
//! `rust,ignore`. This script reads `crates/macros/cgp-fork-macro/src/lib.rs`,
//! takes every ```` ```rust,ignore ```` block, and writes them as ordinary
//! ```` ```rust ```` blocks to `$OUT_DIR/macro_examples.md`, each preceded by a
//! hidden `use cgp_fork::prelude::*;`. `src/lib.rs` includes that file as the
//! docs of a `#[cfg(doctest)]` item, so `cargo test -p cgp-fork-doctests --doc`
//! compiles every example.
//!
//! An example that cannot compile on its own (an abridged expansion, a grammar
//! sketch) opts out with a hidden first line naming the reason:
//!
//! ```text
//! # // doc-example: not compiled (abridged expansion)
//! ```
//!
//! The reason is required; a bare marker fails the build. Lines starting with
//! `# ` are hidden by rustdoc, so neither the setup nor the marker shows on
//! docs.rs.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::{env, fs};

const MARKER: &str = "# // doc-example: not compiled (";

struct Example {
    owner: String,
    index: usize,
    line: usize,
    code: String,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source_path = manifest_dir.join("../../macros/cgp-fork-macro/src/lib.rs");
    println!("cargo::rerun-if-changed={}", source_path.display());
    println!("cargo::rerun-if-changed=build.rs");

    let source = fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("reading {}: {error}", source_path.display()));

    let mut examples = Vec::new();
    let mut pending: Vec<(usize, Vec<String>)> = Vec::new();
    let mut current: Option<(usize, Vec<String>)> = None;

    for (number, raw) in source.lines().enumerate() {
        let line_number = number + 1;
        // Doc text is either `/** ... */` (indented) or `/// ...`.
        let text = raw.trim_start().strip_prefix("///").unwrap_or(raw);
        let trimmed = text.trim();

        if let Some((_, lines)) = current.as_mut() {
            if trimmed == "```" {
                pending.push(current.take().unwrap());
            } else {
                lines.push(text.to_owned());
            }
            continue;
        }

        if trimmed == "```rust,ignore" {
            current = Some((line_number, Vec::new()));
            continue;
        }

        if let Some(name) = item_name(trimmed) {
            for (index, (line, lines)) in pending.drain(..).enumerate() {
                examples.push(Example {
                    owner: name.clone(),
                    index,
                    line,
                    code: dedent(&lines),
                });
            }
        }
    }
    assert!(
        current.is_none(),
        "unterminated ```rust,ignore block in {}",
        source_path.display()
    );
    assert!(
        pending.is_empty(),
        "examples after the last item in {}",
        source_path.display()
    );

    let mut compiled = 0;
    let mut skipped = Vec::new();
    let mut out =
        String::from("Examples from the `cgp-fork-macro` docs, compiled against `cgp-fork`.\n\n");

    for example in &examples {
        let first = example.code.lines().next().unwrap_or("").trim();
        if let Some(rest) = first.strip_prefix(MARKER) {
            let reason = rest.strip_suffix(')').unwrap_or("").trim();
            assert!(
                !reason.is_empty(),
                "cgp-fork-macro/src/lib.rs:{}: `{}` example #{} has a doc-example marker without a reason",
                example.line,
                example.owner,
                example.index,
            );
            skipped.push(format!("{} #{} ({reason})", example.owner, example.index));
            continue;
        }
        assert!(
            !example.code.contains("doc-example:"),
            "cgp-fork-macro/src/lib.rs:{}: the doc-example marker must be the block's first line",
            example.line,
        );

        compiled += 1;
        writeln!(
            out,
            "`{}` example #{} (cgp-fork-macro/src/lib.rs:{}):\n\n```rust\n# #![allow(unused)]\n# use cgp_fork::prelude::*;\n{}\n```\n",
            example.owner, example.index, example.line, example.code,
        )
        .unwrap();
    }

    writeln!(out, "Not compiled ({}):\n", skipped.len()).unwrap();
    for entry in &skipped {
        writeln!(out, "- {entry}").unwrap();
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("macro_examples.md"), out).unwrap();
    // A machine-readable count, for the summary test in `tests/` and for CI logs.
    fs::write(
        out_dir.join("macro_examples_count.txt"),
        format!(
            "total={} compiled={compiled} skipped={}\n",
            examples.len(),
            skipped.len()
        ),
    )
    .unwrap();
}

/// The name of a `pub fn` / `pub struct` item line, which owns the examples above it.
fn item_name(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("pub fn ")
        .or_else(|| line.strip_prefix("pub struct "))?;
    let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
    Some(rest[..end].to_owned())
}

fn dedent(lines: &[String]) -> String {
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or("").trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}
