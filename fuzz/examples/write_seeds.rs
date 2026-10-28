//! Write the starting corpus from the robustness seeds:
//! `cargo +nightly run --manifest-path fuzz/Cargo.toml --example write_seeds -- fuzz/corpus/entrypoints`
fn main() -> std::io::Result<()> {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "corpus/entrypoints".into());
    let written = cgp_fork_macro_fuzz::write_seed_corpus(std::path::Path::new(&dir))?;
    println!("wrote {written} seed inputs to {dir}");
    Ok(())
}
