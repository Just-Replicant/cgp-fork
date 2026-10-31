//! Each file under `tests/renamed/` is a facade behavior test plus one header line.
//! The rest must be that source with every `cgp_fork` renamed to `cgp`, so the copies
//! cannot drift from the tests they stand in for. Whitespace and trailing commas are
//! ignored: rustfmt wraps `cgp_fork::…` onto more lines than `cgp::…` and then inserts
//! a trailing comma.

use std::path::Path;

fn normalize(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn comparable(text: &str) -> String {
    let chars: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();
    let mut out = String::new();
    for (index, c) in chars.iter().copied().enumerate() {
        if c == ',' && matches!(chars.get(index + 1), Some('}' | ')' | ']')) {
            continue;
        }
        out.push(c);
    }
    out
}

#[test]
fn renamed_copies_differ_only_by_the_crate_name() {
    let package = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = package.join("../../..");
    let renamed = package.join("tests/renamed");
    let mut seen = 0;

    for entry in std::fs::read_dir(&renamed).expect("tests/renamed") {
        let path = entry.expect("directory entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        seen += 1;
        let display = path.display().to_string();
        let copy = normalize(&std::fs::read_to_string(&path).expect("read copy"));
        let (header, body) = copy
            .split_once('\n')
            .unwrap_or_else(|| panic!("{display} has no header line"));
        let source = header
            .strip_prefix("// Copied from `")
            .and_then(|rest| rest.split_once('`'))
            .map(|(relative, _)| relative)
            .unwrap_or_else(|| panic!("{display} header does not name its source"));
        assert!(
            header.ends_with("with `cgp_fork` renamed to `cgp`."),
            "{display}"
        );
        let original = normalize(
            &std::fs::read_to_string(repo.join(source))
                .unwrap_or_else(|error| panic!("source `{source}` of {display}: {error}")),
        );
        assert_eq!(
            comparable(body),
            comparable(&original.replace("cgp_fork", "cgp")),
            "{display}"
        );
    }

    assert!(seen > 0, "tests/renamed has no Rust files");
}
