//! Entrypoint for the `parser_rejections` failure-case target.
//!
//! This target collects inputs that the CGP macros must **reject** — malformed
//! syntax, or forms CGP deliberately disallows. Each case asserts that the
//! relevant `macro_core` parser (or a `macro_lib` entrypoint) returns an
//! error rather than silently accepting the input.
#![allow(dead_code)]

pub mod parser_rejections;
