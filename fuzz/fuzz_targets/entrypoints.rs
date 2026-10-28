//! Fuzz every `cgp-fork-macro` entrypoint. See `cgp_fork_macro_fuzz::run` for
//! the input format.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    cgp_fork_macro_fuzz::run(data);
});
