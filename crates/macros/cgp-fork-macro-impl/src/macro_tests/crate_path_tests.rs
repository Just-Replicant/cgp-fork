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
fn an_unrenamed_dependency_is_the_default_name() {
    let dir = TempDir::new("plain");
    let plain = dir.write("plain/Cargo.toml", "[dependencies]\ncgp-fork = \"0.10\"\n");
    assert_eq!(resolve_crate_name(&plain).as_deref(), Some("cgp_fork"));
    assert_eq!(resolve_crate_name(&dir.0.join("does-not-exist")), None);
}

#[test]
fn a_normal_dependency_beats_a_dev_dependency_rename() {
    let dir = TempDir::new("rank");
    let crate_dir = dir.write(
        "Cargo.toml",
        "[dev-dependencies]\ncgp = { package = \"cgp-fork\" }\n\n[dependencies]\ncgp-fork = \"0.10\"\n",
    );
    assert_eq!(resolve_crate_name(&crate_dir).as_deref(), Some("cgp_fork"));
}

#[test]
fn a_dev_dependency_is_used_when_it_is_the_only_one() {
    let name = crate::crate_path::select_from_sources(
        "[dev-dependencies]\ncgp = { package = \"cgp-fork\" }\n",
        None,
        false,
        &|_| false,
    );
    assert_eq!(name.as_deref(), Some("cgp"));
}

#[test]
fn a_build_dependency_is_used_only_for_a_build_script() {
    let manifest = "[dependencies]\ncgp-fork = \"0.10\"\n[build-dependencies]\ncgp = { package = \"cgp-fork\" }\n";
    assert_eq!(
        crate::crate_path::select_from_sources(manifest, None, false, &|_| false).as_deref(),
        Some("cgp_fork")
    );
    assert_eq!(
        crate::crate_path::select_from_sources(manifest, None, true, &|_| false).as_deref(),
        Some("cgp")
    );
}

#[test]
fn a_target_specific_rename_counts_only_when_its_cfg_matches() {
    let manifest = "[target.'cfg(windows)'.dependencies]\ncgp = { package = \"cgp-fork\" }\n";
    assert_eq!(
        crate::crate_path::select_from_sources(manifest, None, false, &|cfg| cfg == "cfg(windows)")
            .as_deref(),
        Some("cgp")
    );
    assert_eq!(
        crate::crate_path::select_from_sources(manifest, None, false, &|_| false),
        None
    );
    let both = "[target.'cfg(windows)'.dependencies]\ncgp = { package = \"cgp-fork\" }\n[dependencies]\ncgp-fork = \"0.10\"\n";
    assert_eq!(
        crate::crate_path::select_from_sources(both, None, false, &|_| true).as_deref(),
        Some("cgp_fork")
    );
}

#[test]
fn a_keyword_rename_is_a_raw_ident() {
    let name = crate::crate_path::select_from_sources(
        "[dependencies]\ntype = { package = \"cgp-fork\" }\n",
        None,
        false,
        &|_| false,
    );
    assert_eq!(name.as_deref(), Some("r#type"));
    let root = CrateRoot::Extern("r#type".to_owned());
    assert_eq!(quote!(#root).to_string(), ":: r#type");
}

#[test]
fn escaped_quotes_do_not_shift_later_fields() {
    let deps = scan_dependencies(
        "[dependencies]\n\
         cgp = { note = \"say \\\"hi\\\", package = \\\"nope\\\"\", package = \"cgp-fork\" }\n",
    );
    assert_eq!(renamed(&deps), Some("cgp"));
    let package = deps
        .iter()
        .find(|dep| dep.key == "cgp")
        .and_then(|dep| dep.package.as_deref());
    assert_eq!(package, Some("cgp-fork"));
}

#[test]
fn cfg_expressions_match_cargo_cfg_variables() {
    let env = |name: &str| match name {
        "CARGO_CFG_UNIX" => Some(String::new()),
        "CARGO_CFG_TARGET_OS" => Some("linux".to_owned()),
        "CARGO_CFG_TARGET_FAMILY" => Some("unix,wasm".to_owned()),
        _ => None,
    };
    let matches = |expr| crate::crate_path::cfg_expr_matches(expr, &env);
    assert!(matches("cfg(unix)"));
    assert!(!matches("cfg(windows)"));
    assert!(matches("cfg(target_os = \"linux\")"));
    assert!(matches("cfg(target_family = \"wasm\")"));
    assert!(matches("cfg(all(unix, target_os = \"linux\"))"));
    assert!(!matches("cfg(any(windows, target_os = \"macos\"))"));
    assert!(matches("cfg(not(windows))"));
    assert!(matches("cfg(all(unix, not(windows)))"));
    assert!(!matches("not a cfg"));
}

#[test]
fn the_cache_rereads_when_the_workspace_manifest_changes() {
    let dir = TempDir::new("cache");
    let workspace = dir.0.join("Cargo.toml");
    std::fs::write(
        &workspace,
        "[workspace]\nmembers = [\"app\"]\n\n[workspace.dependencies]\ncgp = { package = \"cgp-fork\", version = \"0.10\" }\n",
    )
    .expect("write workspace");
    let app = dir.write(
        "app/Cargo.toml",
        "[package]\nname = \"app\"\n\n[dependencies]\ncgp = { workspace = true }\n",
    );
    assert_eq!(
        crate::crate_path::cached_root_for(&app),
        CrateRoot::Extern("cgp".to_owned())
    );
    std::fs::write(
        &workspace,
        "[workspace]\nmembers = [\"app\"]\n\n[workspace.dependencies]\nother = { version = \"0.10\" }\n",
    )
    .expect("rewrite workspace");
    let file = std::fs::File::options()
        .write(true)
        .open(&workspace)
        .expect("open workspace");
    file.set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(5))
        .expect("bump mtime");
    assert_eq!(
        crate::crate_path::cached_root_for(&app),
        CrateRoot::Extern(DEFAULT_CRATE.to_owned())
    );
}

#[test]
fn a_preset_macro_takes_the_callers_facade_path() {
    let preset = crate::macro_lib::cgp_preset(quote!(Types { type Name = String, })).unwrap();
    let preset = preset.to_string();
    assert!(
        preset.contains("$ for_each : path"),
        "with_components! should take the caller's cgp_for_each path: {preset}"
    );
    assert!(
        preset.contains("$ for_each !"),
        "cgp_for_each should be invoked from that path: {preset}"
    );

    let table = crate::macro_lib::delegate_components(quote!(App { preset Types, })).unwrap();
    let table = table.to_string();
    assert!(
        table.contains(":: cgp_fork :: prelude :: cgp_for_each"),
        "delegate_components! should pass the caller's facade path: {table}"
    );
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
    assert_eq!(
        override_root("r#type"),
        Some(CrateRoot::Extern("r#type".to_owned()))
    );
    assert_eq!(
        override_root("type"),
        Some(CrateRoot::Extern("r#type".to_owned()))
    );
    let root = override_root("r#type").expect("raw ident");
    assert_eq!(quote!(#root).to_string(), ":: r#type");
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
