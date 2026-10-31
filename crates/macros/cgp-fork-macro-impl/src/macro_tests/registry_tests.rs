//! The entrypoint registry is the single list both fuzzers use. These tests keep
//! it consistent with itself and with the cargo-fuzz crate in `fuzz/`.

use std::collections::BTreeSet;
use std::path::Path;

use crate::registry::{entrypoints, fuzz_target_name, pipelines};

#[test]
fn entrypoint_names_are_unique() {
    let names: Vec<_> = entrypoints().into_iter().map(|(name, _)| name).collect();
    let unique: BTreeSet<_> = names.iter().copied().collect();
    assert_eq!(
        names.len(),
        unique.len(),
        "duplicate entrypoint in {names:?}"
    );
}

#[test]
fn fuzz_target_names_are_unique_ignoring_case() {
    let targets: BTreeSet<_> = entrypoints()
        .into_iter()
        .map(|(name, _)| fuzz_target_name(name).to_ascii_lowercase())
        .collect();
    assert_eq!(targets.len(), entrypoints().len());
    assert!(
        !targets.contains("entrypoints"),
        "clashes with the all-entrypoints target"
    );
}

#[test]
fn fuzz_target_name_prefixes_type_level_macros() {
    assert_eq!(fuzz_target_name("Product"), "type_product");
    assert_eq!(fuzz_target_name("product"), "product");
    assert_eq!(fuzz_target_name("cgp_component"), "cgp_component");
}

/// Every entrypoint has a cargo-fuzz target, and there is no stale one. Regenerate with
/// `cargo run --manifest-path fuzz/Cargo.toml --example write_targets`.
///
/// Skipped in a packaged crate, which does not contain the repository's `fuzz/`.
#[test]
fn every_entrypoint_has_a_fuzz_target() {
    let fuzz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fuzz");
    let Ok(manifest) = std::fs::read_to_string(fuzz.join("Cargo.toml")) else {
        return;
    };

    let expected: BTreeSet<String> = entrypoints()
        .into_iter()
        .map(|(name, _)| fuzz_target_name(name))
        .chain(["entrypoints".to_owned()])
        .collect();

    let on_disk: BTreeSet<String> = std::fs::read_dir(fuzz.join("fuzz_targets"))
        .expect("fuzz/fuzz_targets exists")
        .map(|entry| entry.expect("readable directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .map(|path| {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        on_disk, expected,
        "fuzz/fuzz_targets is out of date; run write_targets"
    );

    for target in &expected {
        assert!(
            manifest.contains(&format!("name  = \"{target}\"")),
            "fuzz/Cargo.toml has no [[bin]] for `{target}`; run write_targets",
        );
    }
}

/// The registry lists exactly the 38 entrypoints of the proc-macro shim, by name.
#[test]
fn registry_names_every_entrypoint() {
    let names: BTreeSet<_> = entrypoints().into_iter().map(|(name, _)| name).collect();
    let expected: BTreeSet<_> = [
        "async_trait",
        "blanket_trait",
        "cgp_auto_error",
        "cgp_auto_getter",
        "cgp_auto_impl",
        "cgp_component",
        "cgp_fn",
        "cgp_getter",
        "cgp_impl",
        "cgp_new_provider",
        "cgp_provider",
        "cgp_type",
        "derive_provider",
        "cgp_computer",
        "cgp_producer",
        "cgp_auto_dispatch",
        "cgp_auto_log",
        "delegate_components",
        "delegate_and_check_components",
        "check_components",
        "cgp_for_each",
        "cgp_preset",
        "cgp_namespace",
        "Symbol",
        "Path",
        "Product",
        "product",
        "Sum",
        "Struct",
        "Enum",
        "derive_has_field",
        "derive_has_fields",
        "derive_build_field",
        "derive_extract_field",
        "derive_from_variant",
        "derive_cgp_data",
        "derive_cgp_record",
        "derive_cgp_variant",
    ]
    .into_iter()
    .collect();
    assert_eq!(names, expected);
}

/// Every IR family is produced by at least one entrypoint.
#[test]
fn every_ir_family_is_used() {
    let used: BTreeSet<_> = pipelines()
        .into_iter()
        .map(|(_, family, _)| family)
        .collect();
    let all: BTreeSet<_> = crate::ir::IrFamily::ALL.into_iter().collect();
    assert_eq!(used, all);
}
