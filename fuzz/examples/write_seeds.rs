//! Write the starting corpora from the robustness seeds, one directory per target:
//! `cargo +nightly run --manifest-path fuzz/Cargo.toml --example write_seeds -- fuzz/corpus`
fn main() -> std::io::Result<()> {
    let root = std::env::args().nth(1).unwrap_or_else(|| "corpus".into());
    let written = cgp_fork_macro_fuzz::write_seed_corpus(std::path::Path::new(&root))?;
    println!("wrote {written} seed inputs under {root}");
    Ok(())
}
