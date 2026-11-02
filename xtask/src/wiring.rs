//! `cargo xtask wiring` prints context → component → provider edges from Rust source.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use cgp_fork_macro_impl::wiring::{GraphFormat, WiringFile, render, scan_source};

pub fn print_help() {
    eprintln!(
        "\
Usage: cargo xtask <command>

  wiring [--format json|mermaid|dot] [PATH...]
      Read delegate_components!, delegate_and_check_components!, and cgp_preset!,
      plus #[cgp_provider] and #[cgp_impl] dependencies, and print one graph.
      An edge is context to component to provider. The default path is the
      current directory, and the default format is mermaid.

  migrate [--check] [--guide] [--explicit-only] [--keep-async-trait] [--crate NAME] [PATH...]
      Rewrite 0.10 handler imports, paths, and attributes to the 1.0 names.
      The default path is the current directory. --check exits with status 1
      when a file would change and writes nothing. --guide prints the migration
      table and does not touch files. --explicit-only skips names brought in by
      a glob import. --crate is the dependency name (default cgp_fork) and may
      be repeated.

  ci [fmt|clippy|test]
      Run the local checks. With no argument, run all three.

  help
      Show this message."
    );
}

pub fn wiring(args: Vec<String>) -> ExitCode {
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

fn collect(path: &Path, files: &mut Vec<WiringFile>) -> Result<(), String> {
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

fn read_file(path: &Path, files: &mut Vec<WiringFile>) -> Result<(), String> {
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
