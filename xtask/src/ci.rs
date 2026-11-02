//! The checks a developer runs before pushing. CI's other jobs (nextest on
//! three operating systems, semver, deny, fuzz) stay in `.github/workflows/tests.yml`.

use std::process::{Command, ExitCode};

pub fn ci(steps: &[String]) -> ExitCode {
    let default = ["fmt".to_owned(), "clippy".to_owned(), "test".to_owned()];
    let steps: &[String] = if steps.is_empty() { &default } else { steps };
    for step in steps {
        let code = match step.as_str() {
            "fmt" => fmt(),
            "clippy" => clippy(),
            "test" => test(),
            other => {
                eprintln!("unknown ci step `{other}`; use fmt, clippy, or test");
                return ExitCode::from(2);
            }
        };
        if code != ExitCode::SUCCESS {
            return code;
        }
    }
    ExitCode::SUCCESS
}

fn fmt() -> ExitCode {
    let workspace = cargo(&["+nightly", "fmt", "--all", "--", "--check"]);
    if workspace != ExitCode::SUCCESS {
        return workspace;
    }
    cargo(&[
        "+nightly",
        "fmt",
        "--manifest-path",
        "fuzz/Cargo.toml",
        "--",
        "--check",
    ])
}

fn clippy() -> ExitCode {
    let all = cargo(&[
        "clippy",
        "--workspace",
        "--all-features",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
    if all != ExitCode::SUCCESS {
        return all;
    }
    cargo(&[
        "clippy",
        "--workspace",
        "--no-default-features",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])
}

fn test() -> ExitCode {
    let all = cargo(&["test", "--workspace", "--all-features"]);
    if all != ExitCode::SUCCESS {
        return all;
    }
    cargo(&["test", "--workspace", "--no-default-features"])
}

fn cargo(args: &[&str]) -> ExitCode {
    let status = Command::new("cargo").args(args).status();
    match status {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => status
            .code()
            .and_then(|code| u8::try_from(code).ok())
            .map_or(ExitCode::FAILURE, ExitCode::from),
        Err(error) => {
            eprintln!("cargo {}: {error}", args.join(" "));
            ExitCode::FAILURE
        }
    }
}
