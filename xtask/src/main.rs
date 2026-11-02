//! `cargo xtask wiring` prints context → component → provider edges from Rust source.
//!
//! The alias is in `.cargo/config.toml`. `cargo run -p xtask -- wiring` is the same command.

use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use cgp_fork_macro_impl::wiring::{GraphFormat, render, scan_source};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("wiring") => wiring(args.collect()),
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("unknown command `{other}`");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    eprintln!(
        "\
Usage: cargo xtask wiring [--format json|mermaid|dot] [PATH...]

Reads every `delegate_components!`, `delegate_and_check_components!`, and
`cgp_preset!`, plus `#[cgp_provider]` and `#[cgp_impl]` dependencies, and prints
one graph. An edge is context to component to provider. Paths may be files or
directories. The default path is the current directory, and the default
format is mermaid."
    );
}

fn wiring(args: Vec<String>) -> ExitCode {
    let mut format = GraphFormat::Mermaid;
    let mut paths = Vec::new();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" | "-f" => {
                let Some(value) = args.next() else {
                    eprintln!("--format needs json, mermaid, or dot");
                    return ExitCode::from(2);
                };
                format = match value.as_str() {
                    "json" => GraphFormat::Json,
                    "mermaid" => GraphFormat::Mermaid,
                    "dot" => GraphFormat::Dot,
                    other => {
                        eprintln!("unknown format `{other}`; use json, mermaid, or dot");
                        return ExitCode::from(2);
                    }
                };
            }
            "--help" | "-h" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            other if other.starts_with('-') => {
                eprintln!("unknown option `{other}`");
                return ExitCode::from(2);
            }
            other => paths.push(other.to_owned()),
        }
    }

    if paths.is_empty() {
        paths.push(".".to_owned());
    }

    let mut files = Vec::new();
    let mut failed = false;

    for path in &paths {
        if let Err(error) = collect(Path::new(path), &mut files) {
            eprintln!("{path}: {error}");
            failed = true;
        }
    }

    if failed {
        return ExitCode::FAILURE;
    }

    println!("{}", render(&files, format));
    ExitCode::SUCCESS
}

fn collect(
    path: &Path,
    files: &mut Vec<cgp_fork_macro_impl::wiring::WiringFile>,
) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|error| error.to_string())?;

    if meta.is_dir() {
        for entry in fs::read_dir(path).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            let child = entry.path();
            if child.is_dir() {
                let name = child
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if matches!(name, "target" | ".git" | "node_modules") {
                    continue;
                }
                collect(&child, files)?;
            } else if child.extension().is_some_and(|ext| ext == "rs") {
                read_file(&child, files)?;
            }
        }
        return Ok(());
    }

    read_file(path, files)
}

fn read_file(
    path: &Path,
    files: &mut Vec<cgp_fork_macro_impl::wiring::WiringFile>,
) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|error| error.to_string())?;
    match scan_source(&source) {
        Ok(mut file) => {
            if !file.tables.is_empty() || !file.presets.is_empty() || !file.providers.is_empty() {
                file.path = path.display().to_string();
                files.push(file);
            }
            Ok(())
        }
        Err(error) => Err(error.to_string()),
    }
}
