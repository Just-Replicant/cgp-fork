//! The path generated code uses to name the `cgp-fork` facade.
//!
//! Generated code refers to items as `::cgp_fork::macro_prelude::…`. That only resolves when the
//! calling crate imports the package `cgp-fork` under its default name. A renamed dependency,
//!
//! ```toml
//! [dependencies]
//! cgp = { package = "cgp-fork", version = "0.10" }
//! ```
//!
//! must be named `::cgp` instead. [`crate_root`] finds that name by reading the calling crate's
//! manifest (`$CARGO_MANIFEST_DIR/Cargo.toml`), and the manifest of its workspace when the
//! dependency is inherited with `workspace = true`. It falls back to `::cgp_fork` whenever it
//! cannot tell, which is also the right answer inside `cgp-fork` itself (its `lib.rs` declares
//! `extern crate self as cgp_fork`).
//!
//! Build systems without Cargo can set the environment variable [`ENV_OVERRIDE`] to a path, such
//! as `cgp` or `::my_reexports::cgp`.
//!
//! See `docs/adr/0003-crate-path-resolution.md` in the repository.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

/// The package name of the facade on crates.io.
pub const PACKAGE: &str = "cgp-fork";

/// The crate name generated code uses when nothing else is known.
pub const DEFAULT_CRATE: &str = "cgp_fork";

/// Environment variable that, when set to a Rust path, overrides the detection.
pub const ENV_OVERRIDE: &str = "CGP_FORK_CRATE";

/// The facade's path as seen from the crate being compiled, e.g. `::cgp_fork` or `::cgp`.
///
/// Implements [`ToTokens`], so it can be interpolated in `quote!` directly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrateRoot {
    /// An extern crate name, emitted as `::name`.
    Extern(String),
    /// A path given through [`ENV_OVERRIDE`], emitted as written.
    Path(String),
}

impl CrateRoot {
    /// The single crate name, when the root is an extern crate.
    pub fn extern_name(&self) -> Option<&str> {
        match self {
            CrateRoot::Extern(name) => Some(name),
            CrateRoot::Path(_) => None,
        }
    }
}

impl ToTokens for CrateRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CrateRoot::Extern(name) => {
                let ident = Ident::new(name, Span::call_site());
                tokens.extend(quote! { :: #ident });
            }
            CrateRoot::Path(path) => match syn::parse_str::<syn::Path>(path) {
                Ok(path) => path.to_tokens(tokens),
                // `override_root` validated the path, so this is unreachable in practice; keep
                // generating code that names the default rather than panicking.
                Err(_) => {
                    let ident = Ident::new(DEFAULT_CRATE, Span::call_site());
                    tokens.extend(quote! { :: #ident });
                }
            },
        }
    }
}

/// The facade's path for the crate being compiled. Cheap to call repeatedly: the manifest is
/// read once per compiler thread and re-read only when its modification time changes.
pub fn crate_root() -> CrateRoot {
    if let Some(root) = std::env::var(ENV_OVERRIDE)
        .ok()
        .as_deref()
        .and_then(override_root)
    {
        return root;
    }
    match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(dir) => cached_root_for(Path::new(&dir)),
        None => CrateRoot::Extern(DEFAULT_CRATE.to_owned()),
    }
}

/// Parse an [`ENV_OVERRIDE`] value. A bare identifier means an extern crate; anything else must
/// parse as a Rust path. An empty or malformed value is ignored.
pub fn override_root(value: &str) -> Option<CrateRoot> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let path: syn::Path = syn::parse_str(value).ok()?;
    if path.leading_colon.is_none() && path.segments.len() == 1 {
        let segment = &path.segments[0];
        let name = segment.ident.to_string();
        if segment.arguments.is_none() && !matches!(name.as_str(), "crate" | "self" | "super") {
            return Some(CrateRoot::Extern(name));
        }
    }
    Some(CrateRoot::Path(value.to_owned()))
}

thread_local! {
    // Keyed by manifest directory: rust-analyzer's proc-macro server expands macros for many
    // crates in one process. Only plain strings are cached; token handles must not outlive the
    // macro invocation that created them.
    static CACHE: RefCell<BTreeMap<PathBuf, (Option<SystemTime>, String)>> =
        const { RefCell::new(BTreeMap::new()) };
}

fn cached_root_for(manifest_dir: &Path) -> CrateRoot {
    let manifest = manifest_dir.join("Cargo.toml");
    let modified = std::fs::metadata(&manifest).and_then(|m| m.modified()).ok();

    let cached = CACHE.with(|cache| {
        cache
            .borrow()
            .get(manifest_dir)
            .filter(|(time, _)| *time == modified)
            .map(|(_, name)| name.clone())
    });
    let name = cached.unwrap_or_else(|| {
        let name = resolve_crate_name(manifest_dir).unwrap_or_else(|| DEFAULT_CRATE.to_owned());
        CACHE.with(|cache| {
            cache
                .borrow_mut()
                .insert(manifest_dir.to_owned(), (modified, name.clone()));
        });
        name
    });
    CrateRoot::Extern(name)
}

/// The name the crate in `manifest_dir` imports `cgp-fork` under, if its manifest (or its
/// workspace's, for `workspace = true`) says so. Uncached.
pub fn resolve_crate_name(manifest_dir: &Path) -> Option<String> {
    let manifest = std::fs::read_to_string(manifest_dir.join("Cargo.toml")).ok()?;
    let deps = scan_dependencies(&manifest);

    if let Some(dep) = deps
        .iter()
        .find(|dep| !dep.workspace_table && dep.package.as_deref() == Some(PACKAGE))
    {
        return valid_crate_name(&dep.key);
    }

    // `cgp = { workspace = true }`: the rename lives in `[workspace.dependencies]`.
    let inherited: Vec<&Dependency> = deps
        .iter()
        .filter(|dep| !dep.workspace_table && dep.inherits)
        .collect();
    if inherited.is_empty() {
        return None;
    }
    let workspace_manifest = find_workspace_manifest(manifest_dir, &manifest)?;
    let workspace_deps = scan_dependencies(&workspace_manifest);
    inherited.iter().find_map(|dep| {
        workspace_deps
            .iter()
            .any(|ws| {
                ws.workspace_table && ws.key == dep.key && ws.package.as_deref() == Some(PACKAGE)
            })
            .then(|| valid_crate_name(&dep.key))
            .flatten()
    })
}

/// The workspace root's manifest: the crate's own manifest if it declares `[workspace]`,
/// otherwise the nearest ancestor `Cargo.toml` that does.
fn find_workspace_manifest(manifest_dir: &Path, own_manifest: &str) -> Option<String> {
    if declares_workspace(own_manifest) {
        return Some(own_manifest.to_owned());
    }
    manifest_dir.ancestors().skip(1).find_map(|dir| {
        let text = std::fs::read_to_string(dir.join("Cargo.toml")).ok()?;
        declares_workspace(&text).then_some(text)
    })
}

fn declares_workspace(manifest: &str) -> bool {
    manifest.lines().any(|line| {
        let line = strip_comment(line).trim().to_owned();
        line == "[workspace]" || line.starts_with("[workspace.")
    })
}

/// A dependency key as a Rust crate name (`-` becomes `_`), if it is a valid identifier.
fn valid_crate_name(key: &str) -> Option<String> {
    let name = key.replace('-', "_");
    syn::parse_str::<Ident>(&name).ok().map(|_| name)
}

/// One dependency entry found in a manifest.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dependency {
    /// The key the dependency is declared under, which becomes the crate name.
    pub key: String,
    /// The `package = "…"` value, when the dependency is renamed.
    pub package: Option<String>,
    /// `workspace = true`: the entry is inherited from `[workspace.dependencies]`.
    pub inherits: bool,
    /// The entry is in `[workspace.dependencies]` rather than a crate's own tables.
    pub workspace_table: bool,
}

/// Where a manifest line sits.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Section {
    Other,
    /// `[dependencies]`, `[target.….dependencies]`, `[workspace.dependencies]`, …
    Table {
        workspace: bool,
    },
    /// `[dependencies.<key>]` and friends.
    Entry {
        key: String,
        workspace: bool,
    },
}

/// Collect the dependency entries of a Cargo manifest.
///
/// This understands the subset of TOML that Cargo manifests use for dependencies: table headers,
/// `key = "version"`, inline tables (also spread over several lines), dotted keys
/// (`cgp.package = "cgp-fork"`), quoted keys, and comments. Anything else is skipped, never an
/// error: the caller falls back to the default crate name.
pub fn scan_dependencies(manifest: &str) -> Vec<Dependency> {
    let mut deps: Vec<Dependency> = Vec::new();
    let mut section = Section::Other;

    for statement in logical_lines(manifest) {
        let statement = statement.trim();
        if statement.starts_with("[[") {
            section = Section::Other;
            continue;
        }
        if let Some(header) = statement
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
        {
            section = classify_header(&split_dotted(header));
            continue;
        }
        let Some((raw_key, value)) = split_key_value(statement) else {
            continue;
        };
        let key_path = split_dotted(raw_key);

        match &section {
            Section::Other => {}
            Section::Table { workspace } => {
                let Some((key, rest)) = key_path.split_first() else {
                    continue;
                };
                let dep = entry(&mut deps, key, *workspace);
                match rest {
                    [] => apply_value(dep, value),
                    [field] => apply_field(dep, field, value),
                    _ => {}
                }
            }
            Section::Entry { key, workspace } => {
                if let [field] = key_path.as_slice() {
                    let dep = entry(&mut deps, key, *workspace);
                    apply_field(dep, field, value);
                }
            }
        }
    }
    deps
}

fn entry<'a>(
    deps: &'a mut Vec<Dependency>,
    key: &str,
    workspace_table: bool,
) -> &'a mut Dependency {
    let position = deps
        .iter()
        .position(|dep| dep.key == key && dep.workspace_table == workspace_table);
    let index = position.unwrap_or_else(|| {
        deps.push(Dependency {
            key: key.to_owned(),
            workspace_table,
            ..Dependency::default()
        });
        deps.len() - 1
    });
    &mut deps[index]
}

fn is_dependency_table(name: &str) -> bool {
    matches!(
        name,
        "dependencies"
            | "dev-dependencies"
            | "dev_dependencies"
            | "build-dependencies"
            | "build_dependencies"
    )
}

fn classify_header(path: &[String]) -> Section {
    let (table, workspace, rest) = match path {
        [first, second, rest @ ..] if first == "workspace" && second == "dependencies" => {
            (second, true, rest)
        }
        [first, _cfg, table, rest @ ..] if first == "target" => (table, false, rest),
        [table, rest @ ..] => (table, false, rest),
        [] => return Section::Other,
    };
    if !is_dependency_table(table) {
        return Section::Other;
    }
    match rest {
        [] => Section::Table { workspace },
        [key] => Section::Entry {
            key: key.clone(),
            workspace,
        },
        _ => Section::Other,
    }
}

/// Apply `key = <value>` where the value is a version string or an inline table.
fn apply_value(dep: &mut Dependency, value: &str) {
    let value = value.trim();
    if let Some(inner) = value.strip_prefix('{').and_then(|v| v.strip_suffix('}')) {
        for item in split_top_level(inner, ',') {
            if let Some((field, field_value)) = split_key_value(item.trim())
                && let [field] = split_dotted(field).as_slice()
            {
                apply_field(dep, field, field_value);
            }
        }
    }
}

fn apply_field(dep: &mut Dependency, field: &str, value: &str) {
    match field {
        "package" => dep.package = unquote(value.trim()),
        "workspace" => dep.inherits = value.trim() == "true",
        _ => {}
    }
}

/// Join physical lines into statements, so an inline table or array spread over several lines
/// is one statement, and drop comments.
fn logical_lines(text: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut depth: i32 = 0;
    for line in text.lines() {
        let line = strip_comment(line);
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(&line);
        depth += bracket_balance(&line);
        if depth <= 0 {
            statements.push(std::mem::take(&mut current));
            depth = 0;
        }
    }
    if !current.is_empty() {
        statements.push(current);
    }
    statements
}

/// The net `{`/`[` nesting a line opens, ignoring quoted text. A table header line
/// (`[dependencies]`) balances to zero.
fn bracket_balance(line: &str) -> i32 {
    let mut balance = 0;
    let mut quote: Option<char> = None;
    for c in line.chars() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '{' | '[') => balance += 1,
            (None, '}' | ']') => balance -= 1,
            _ => {}
        }
    }
    balance
}

/// Remove a `#` comment that is not inside a string.
fn strip_comment(line: &str) -> String {
    let mut quote: Option<char> = None;
    for (index, c) in line.char_indices() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '#') => return line[..index].to_owned(),
            _ => {}
        }
    }
    line.to_owned()
}

/// Split `key = value` at the first `=` outside quotes.
fn split_key_value(statement: &str) -> Option<(&str, &str)> {
    let mut quote: Option<char> = None;
    for (index, c) in statement.char_indices() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '=') => return Some((statement[..index].trim(), &statement[index + 1..])),
            _ => {}
        }
    }
    None
}

/// Split at `separator` outside quotes and outside nested `{}`/`[]`.
fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut quote: Option<char> = None;
    let mut depth = 0i32;
    let mut start = 0;
    for (index, c) in text.char_indices() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '{' | '[') => depth += 1,
            (None, '}' | ']') => depth -= 1,
            (None, c) if c == separator && depth == 0 => {
                parts.push(&text[start..index]);
                start = index + c.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&text[start..]);
    parts
}

/// Split a dotted key (`target.'cfg(unix)'.dependencies`) into unquoted segments.
fn split_dotted(key: &str) -> Vec<String> {
    split_top_level(key, '.')
        .into_iter()
        .map(|segment| {
            let segment = segment.trim();
            unquote(segment).unwrap_or_else(|| segment.to_owned())
        })
        .collect()
}

/// The contents of a `"…"` or `'…'` string (escapes are not needed for crate names).
fn unquote(value: &str) -> Option<String> {
    let value = value.trim();
    ['"', '\''].iter().find_map(|&q| {
        value
            .strip_prefix(q)
            .and_then(|v| v.strip_suffix(q))
            .map(str::to_owned)
    })
}
