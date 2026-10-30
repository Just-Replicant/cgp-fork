//! How generated code names the facade when the dependency is renamed
//! (`crate_path`, ADR 0003). The end-to-end check is the `cgp-fork-renamed-dep`
//! workspace crate, which imports `cgp-fork` as `cgp`.

use std::path::{Path, PathBuf};

use quote::quote;

use crate::crate_path::{
    CrateRoot, DEFAULT_CRATE, Dependency, override_root, resolve_crate_name, scan_dependencies,
};

fn renamed(deps: &[Dependency]) -> Option<&str> {
    deps.iter()
        .find(|dep| !dep.workspace_table && dep.package.as_deref() == Some("cgp-fork"))
        .map(|dep| dep.key.as_str())
}

#[test]
fn finds_an_inline_table_rename() {
    let deps = scan_dependencies(
        r#"
[package]
name = "app"

[dependencies]
serde = "1"
cgp = { package = "cgp-fork", version = "0.10", features = ["anyhow", "eyre"] } # renamed
"#,
    );
    assert_eq!(renamed(&deps), Some("cgp"));
}

#[test]
fn finds_a_dependency_subtable_rename() {
    let deps = scan_dependencies(
        r#"
[dependencies.my-cgp]
version = "0.10"
package = 'cgp-fork'
"#,
    );
    assert_eq!(renamed(&deps), Some("my-cgp"));
}

#[test]
fn finds_a_dotted_key_rename() {
    let deps = scan_dependencies(
        "[dev-dependencies]\ncgp.package = \"cgp-fork\"\ncgp.version = \"0.10\"\n",
    );
    assert_eq!(renamed(&deps), Some("cgp"));
}

#[test]
fn finds_a_target_specific_rename() {
    let deps = scan_dependencies(
        "[target.'cfg(unix)'.dependencies]\n\"cgp\" = { package = \"cgp-fork\" }\n",
    );
    assert_eq!(renamed(&deps), Some("cgp"));
}

#[test]
fn finds_a_multi_line_inline_table() {
    let deps = scan_dependencies(
        "[dependencies]\ncgp = {\n  package = \"cgp-fork\",\n  features = [\n    \"anyhow\",\n  ],\n}\n",
    );
    assert_eq!(renamed(&deps), Some("cgp"));
}

#[test]
fn ignores_package_in_other_tables_and_comments() {
    let deps = scan_dependencies(
        r#"
[package]
name = "cgp-fork"
# cgp = { package = "cgp-fork" }

[features]
cgp = []

[dependencies]
other = { package = "not-cgp", version = "1" }
"#,
    );
    assert_eq!(renamed(&deps), None);
}

#[test]
fn records_workspace_inheritance() {
    let deps = scan_dependencies(
        "[workspace.dependencies]\ncgp = { package = \"cgp-fork\", version = \"0.10\" }\n\n[dependencies]\ncgp = { workspace = true }\n",
    );
    assert!(deps.iter().any(|dep| dep.workspace_table
        && dep.key == "cgp"
        && dep.package.as_deref() == Some("cgp-fork")));
    assert!(
        deps.iter()
            .any(|dep| !dep.workspace_table && dep.key == "cgp" && dep.inherits)
    );
}

#[test]
fn malformed_manifests_find_nothing_and_do_not_panic() {
    for text in [
        "[dependencies",
        "cgp = {",
        "= = =",
        "[dependencies]\ncgp = { package = }",
        "\u{0}\u{1}",
        "",
    ] {
        assert_eq!(renamed(&scan_dependencies(text)), None, "{text:?}");
    }
}

/// A fresh directory under the system temp dir, removed on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("cgp-fork-crate-path-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn write(&self, relative: &str, text: &str) -> PathBuf {
        let path = self.0.join(relative);
        std::fs::create_dir_all(path.parent().expect("has parent")).expect("create dirs");
        std::fs::write(&path, text).expect("write manifest");
        path.parent().expect("has parent").to_owned()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn resolves_a_direct_rename_with_dashes() {
    let dir = TempDir::new("direct");
    let crate_dir = dir.write(
        "Cargo.toml",
        "[dependencies]\nmy-cgp = { package = \"cgp-fork\" }\n",
    );
    assert_eq!(resolve_crate_name(&crate_dir).as_deref(), Some("my_cgp"));
}

#[test]
fn resolves_a_rename_inherited_from_the_workspace() {
    let dir = TempDir::new("workspace");
    dir.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"app\"]\n\n[workspace.dependencies]\ncgp = { package = \"cgp-fork\", version = \"0.10\" }\n",
    );
    let app = dir.write(
        "app/Cargo.toml",
        "[package]\nname = \"app\"\n\n[dependencies]\ncgp.workspace = true\n",
    );
    assert_eq!(resolve_crate_name(&app).as_deref(), Some("cgp"));
}

#[test]
fn an_unrenamed_or_missing_dependency_resolves_to_nothing() {
    let dir = TempDir::new("plain");
    let plain = dir.write("plain/Cargo.toml", "[dependencies]\ncgp-fork = \"0.10\"\n");
    assert_eq!(resolve_crate_name(&plain), None);
    assert_eq!(resolve_crate_name(&dir.0.join("does-not-exist")), None);
}

#[test]
fn this_crate_resolves_to_the_default() {
    // The impl crate's own manifest does not depend on `cgp-fork`.
    assert_eq!(
        resolve_crate_name(Path::new(env!("CARGO_MANIFEST_DIR"))),
        None
    );
    assert_eq!(
        crate::crate_path::crate_root(),
        CrateRoot::Extern(DEFAULT_CRATE.to_owned())
    );
}

#[test]
fn parses_the_environment_override() {
    assert_eq!(
        override_root("cgp"),
        Some(CrateRoot::Extern("cgp".to_owned()))
    );
    assert_eq!(override_root("  "), None);
    assert_eq!(override_root("not a path"), None);
    assert_eq!(
        override_root("::reexports::cgp"),
        Some(CrateRoot::Path("::reexports::cgp".to_owned()))
    );
    assert_eq!(
        override_root("crate"),
        Some(CrateRoot::Path("crate".to_owned()))
    );
}

#[test]
fn crate_roots_render_as_paths() {
    let root = CrateRoot::Extern("cgp".to_owned());
    assert_eq!(
        quote!(#root::macro_prelude::HasField).to_string(),
        ":: cgp :: macro_prelude :: HasField"
    );
    let root = CrateRoot::Path("::reexports::cgp".to_owned());
    assert_eq!(
        quote!(#root::prelude).to_string(),
        ":: reexports :: cgp :: prelude"
    );
}
