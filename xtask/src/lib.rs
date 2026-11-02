//! Repository tasks: `cargo xtask wiring`, `cargo xtask migrate`, and `cargo xtask ci`.

mod ci;
mod migrate;
mod wiring;

use std::env;
use std::process::ExitCode;

pub use migrate::migration_table;

/// Runs the xtask command line.
pub fn run() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("wiring") => wiring::wiring(args.collect()),
        Some("migrate") => migrate::migrate_command(args.collect()),
        Some("ci") => ci::ci(&args.collect::<Vec<_>>()),
        Some("help") | Some("--help") | Some("-h") | None => {
            wiring::print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("unknown command `{other}`");
            wiring::print_help();
            ExitCode::from(2)
        }
    }
}
