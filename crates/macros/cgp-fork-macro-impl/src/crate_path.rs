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
//! When several tables name the package, a normal `[dependencies]` entry wins over a
//! target-specific one, then a dev-dependency. Build-dependencies are used only while compiling
//! a build script. A target-specific table counts only when its `cfg(...)` matches this build.
//! A key of `cgp-fork` with no `package` field is that crate, so it beats a rename that lives
//! only in another table.
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
                let ident = extern_ident(name);
                tokens.extend(quote! { :: #ident });
            }
            CrateRoot::Path(path) => match syn::parse_str::<syn::Path>(path) {
                Ok(path) => path.to_tokens(tokens),
                // `override_root` validated the path, so this is unreachable in practice; keep
                // generating code that names the default rather than panicking.
                Err(_) => {
                    let ident = extern_ident(DEFAULT_CRATE);
                    tokens.extend(quote! { :: #ident });
                }
            },
        }
    }
}

/// An extern-crate ident, including raw idents (`r#type`). Never panics: a string that is not
/// an ident becomes [`DEFAULT_CRATE`].
fn extern_ident(name: &str) -> Ident {
    if let Ok(ident) = syn::parse_str::<Ident>(name) {
        return ident;
    }
    if let Ok(ident) = syn::parse_str::<Ident>(&format!("r#{name}")) {
        return ident;
    }
    syn::parse_str::<Ident>(DEFAULT_CRATE)
        .unwrap_or_else(|_| Ident::new(DEFAULT_CRATE, Span::call_site()))
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
    if let Ok(path) = syn::parse_str::<syn::Path>(value) {
        if path.leading_colon.is_none() && path.segments.len() == 1 {
            let segment = &path.segments[0];
            let name = segment.ident.to_string();
            if segment.arguments.is_none() && !matches!(name.as_str(), "crate" | "self" | "super") {
                // `type` does not parse as a path; `r#type` does, and `to_string` keeps the `r#`.
                return valid_crate_name(&name).map(CrateRoot::Extern);
            }
        }
        return Some(CrateRoot::Path(value.to_owned()));
    }
    // A bare keyword (`CGP_FORK_CRATE=type`) is not a Rust path. Emit it as a raw ident.
    valid_crate_name(value).map(CrateRoot::Extern)
}

type CacheEntry = (Option<SystemTime>, Option<SystemTime>, String);

thread_local! {
    // Keyed by manifest directory: rust-analyzer's proc-macro server expands macros for many
    // crates in one process. Only plain strings are cached; token handles must not outlive the
    // macro invocation that created them. The workspace manifest's mtime is part of the key
    // because a `workspace = true` rename lives there, and editing it does not touch the member.
    static CACHE: RefCell<BTreeMap<PathBuf, CacheEntry>> = const { RefCell::new(BTreeMap::new()) };
}

fn file_mtime(path: &Path) -> Option<SystemTime> {
    std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()
}

pub(crate) fn cached_root_for(manifest_dir: &Path) -> CrateRoot {
    let manifest = manifest_dir.join("Cargo.toml");
    let modified = file_mtime(&manifest);
    let own = std::fs::read_to_string(&manifest).unwrap_or_default();
    let workspace = workspace_manifest_path(manifest_dir, &own);
    let workspace_modified = workspace.as_ref().and_then(|path| file_mtime(path));

    let cached = CACHE.with(|cache| {
        cache
            .borrow()
            .get(manifest_dir)
            .and_then(|(member, workspace_time, name)| {
                (*member == modified && *workspace_time == workspace_modified).then(|| name.clone())
            })
    });
    let name = cached.unwrap_or_else(|| {
        let name = resolve_crate_name(manifest_dir).unwrap_or_else(|| DEFAULT_CRATE.to_owned());
        CACHE.with(|cache| {
            cache.borrow_mut().insert(
                manifest_dir.to_owned(),
                (modified, workspace_modified, name.clone()),
            );
        });
        name
    });
    CrateRoot::Extern(name)
}

/// The name the crate in `manifest_dir` imports `cgp-fork` under, if its manifest (or its
/// workspace's, for `workspace = true`) says so. Uncached.
///
/// An unrenamed `cgp-fork` dependency is `Some("cgp_fork")`, not [`None`]: it has to beat a
/// rename that appears only under `[dev-dependencies]` or a target table.
pub fn resolve_crate_name(manifest_dir: &Path) -> Option<String> {
    let manifest = std::fs::read_to_string(manifest_dir.join("Cargo.toml")).ok()?;
    let workspace = workspace_manifest_path(manifest_dir, &manifest)
        .and_then(|path| std::fs::read_to_string(path).ok());
    select_from_sources(
        &manifest,
        workspace.as_deref(),
        std::env::var("CARGO_CRATE_NAME").as_deref() == Ok("build_script_build"),
        &cfg_expr_active,
    )
}

/// Which dependency table wins, given the tables in `manifest` and (when the crate inherits
/// dependencies) the text of the workspace manifest. `cfg_active` receives the `cfg(...)`
/// expression from a `[target.'cfg(...)'.…]` header.
pub(crate) fn select_from_sources(
    manifest: &str,
    workspace_manifest: Option<&str>,
    build_script: bool,
    cfg_active: &dyn Fn(&str) -> bool,
) -> Option<String> {
    let deps = scan_dependencies(manifest);
    let workspace_deps = workspace_manifest
        .map(scan_dependencies)
        .unwrap_or_default();
    let mut best: Option<(u8, String)> = None;
    for dep in &deps {
        let Some(rank) = dep_rank(dep, build_script, cfg_active) else {
            continue;
        };
        let Some(name) = imported_name(dep, &workspace_deps) else {
            continue;
        };
        if best.as_ref().is_none_or(|(best_rank, _)| rank < *best_rank) {
            best = Some((rank, name));
        }
    }
    best.map(|(_, name)| name)
}

/// Lower is a better match. `None` means this entry cannot be linked into the crate being
/// compiled.
fn dep_rank(dep: &Dependency, build_script: bool, cfg_active: &dyn Fn(&str) -> bool) -> Option<u8> {
    if dep.workspace_table {
        return None;
    }
    if let Some(cfg) = &dep.target_cfg
        && !cfg_active(cfg)
    {
        return None;
    }
    if build_script {
        return match dep.table {
            DepTable::Build if dep.target_cfg.is_none() => Some(0),
            DepTable::Build => Some(1),
            DepTable::Normal | DepTable::Dev => None,
        };
    }
    match dep.table {
        DepTable::Normal if dep.target_cfg.is_none() => Some(0),
        DepTable::Normal => Some(1),
        DepTable::Dev if dep.target_cfg.is_none() => Some(2),
        DepTable::Dev => Some(3),
        DepTable::Build => None,
    }
}

/// The extern name `dep` gives this package, following `workspace = true` to `workspace_deps`.
fn imported_name(dep: &Dependency, workspace_deps: &[Dependency]) -> Option<String> {
    if dep.workspace_table {
        return None;
    }
    let is_ours = if dep.inherits {
        let workspace = workspace_deps
            .iter()
            .find(|candidate| candidate.workspace_table && candidate.key == dep.key)?;
        dep.package.as_deref() == Some(PACKAGE) || names_our_package(workspace)
    } else {
        names_our_package(dep)
    };
    is_ours.then(|| valid_crate_name(&dep.key)).flatten()
}

/// `package = "cgp-fork"`, or a key of `cgp-fork` with no `package` field (the key is the package).
fn names_our_package(dep: &Dependency) -> bool {
    match dep.package.as_deref() {
        Some(package) => package == PACKAGE,
        None => dep.key == PACKAGE,
    }
}

/// The workspace root's manifest path: the crate's own manifest if it declares `[workspace]`,
/// otherwise the nearest ancestor `Cargo.toml` that does.
fn workspace_manifest_path(manifest_dir: &Path, own_manifest: &str) -> Option<PathBuf> {
    if declares_workspace(own_manifest) {
        return Some(manifest_dir.join("Cargo.toml"));
    }
    manifest_dir.ancestors().skip(1).find_map(|dir| {
        let path = dir.join("Cargo.toml");
        let text = std::fs::read_to_string(&path).ok()?;
        declares_workspace(&text).then_some(path)
    })
}

/// Whether a `[target.'cfg(...)'.…]` expression matches the `CARGO_CFG_*` variables Cargo sets
/// for this rustc invocation.
fn cfg_expr_active(expr: &str) -> bool {
    cfg_expr_matches(expr, &|name| std::env::var(name).ok())
}

/// `expr` is the header segment, such as `cfg(unix)` or `cfg(all(windows, target_os = "windows"))`.
/// `env` looks up `CARGO_CFG_*` and returns `None` when the variable is unset.
pub(crate) fn cfg_expr_matches(expr: &str, env: &dyn Fn(&str) -> Option<String>) -> bool {
    let Some(inner) = expr
        .trim()
        .strip_prefix("cfg(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return false;
    };
    eval_cfg_predicate(inner.trim(), env)
}

fn eval_cfg_predicate(input: &str, env: &dyn Fn(&str) -> Option<String>) -> bool {
    let input = input.trim();
    if let Some(rest) = strip_cfg_call(input, "all") {
        return rest.trim().is_empty()
            || split_top_level(rest, ',')
                .into_iter()
                .all(|part| eval_cfg_predicate(part.trim(), env));
    }
    if let Some(rest) = strip_cfg_call(input, "any") {
        return !rest.trim().is_empty()
            && split_top_level(rest, ',')
                .into_iter()
                .any(|part| eval_cfg_predicate(part.trim(), env));
    }
    if let Some(rest) = strip_cfg_call(input, "not") {
        return !eval_cfg_predicate(rest.trim(), env);
    }
    if let Some((name, value)) = split_key_value(input) {
        let expected = unquote(value.trim()).unwrap_or_else(|| value.trim().to_owned());
        let Some(actual) = env(&cfg_env_name(name.trim())) else {
            return false;
        };
        return actual.split(',').any(|item| item.trim() == expected);
    }
    env(&cfg_env_name(input)).is_some()
}

fn strip_cfg_call<'a>(input: &'a str, name: &str) -> Option<&'a str> {
    input
        .strip_prefix(name)?
        .strip_prefix('(')?
        .strip_suffix(')')
}

fn cfg_env_name(cfg_name: &str) -> String {
    let mut name = String::from("CARGO_CFG_");
    for c in cfg_name.chars() {
        name.push(match c {
            '-' => '_',
            other => other.to_ascii_uppercase(),
        });
    }
    name
}

fn declares_workspace(manifest: &str) -> bool {
    manifest.lines().any(|line| {
        let line = strip_comment(line).trim().to_owned();
        line == "[workspace]" || line.starts_with("[workspace.")
    })
}

/// A dependency key as a Rust crate name (`-` becomes `_`). A keyword (`type`) is kept as a raw
/// ident (`r#type`), which [`extern_ident`] can emit without panicking.
fn valid_crate_name(key: &str) -> Option<String> {
    let name = key.replace('-', "_");
    if syn::parse_str::<Ident>(&name).is_ok() {
        return Some(name);
    }
    let raw = format!("r#{name}");
    syn::parse_str::<Ident>(&raw).ok().map(|_| raw)
}

/// Which dependency table an entry came from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DepTable {
    /// `[dependencies]`, including target-specific and workspace tables.
    #[default]
    Normal,
    /// `[dev-dependencies]`.
    Dev,
    /// `[build-dependencies]`.
    Build,
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
    /// `[dependencies]`, `[dev-dependencies]`, or `[build-dependencies]`.
    pub table: DepTable,
    /// The `cfg(...)` expression when the entry is under `[target.'cfg(...)'.…]`.
    pub target_cfg: Option<String>,
}

/// Where a manifest line sits.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Section {
    Other,
    /// `[dependencies]`, `[target.….dependencies]`, `[workspace.dependencies]`, …
    Table {
        workspace: bool,
        table: DepTable,
        target_cfg: Option<String>,
    },
    /// `[dependencies.<key>]` and friends.
    Entry {
        key: String,
        workspace: bool,
        table: DepTable,
        target_cfg: Option<String>,
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
            Section::Table {
                workspace,
                table,
                target_cfg,
            } => {
                let Some((key, rest)) = key_path.split_first() else {
                    continue;
                };
                let dep = entry(&mut deps, key, *workspace, *table, target_cfg.as_deref());
                match rest {
                    [] => apply_value(dep, value),
                    [field] => apply_field(dep, field, value),
                    _ => {}
                }
            }
            Section::Entry {
                key,
                workspace,
                table,
                target_cfg,
            } => {
                if let [field] = key_path.as_slice() {
                    let dep = entry(&mut deps, key, *workspace, *table, target_cfg.as_deref());
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
    table: DepTable,
    target_cfg: Option<&str>,
) -> &'a mut Dependency {
    let position = deps.iter().position(|dep| {
        dep.key == key
            && dep.workspace_table == workspace_table
            && dep.table == table
            && dep.target_cfg.as_deref() == target_cfg
    });
    let index = position.unwrap_or_else(|| {
        deps.push(Dependency {
            key: key.to_owned(),
            workspace_table,
            table,
            target_cfg: target_cfg.map(str::to_owned),
            ..Dependency::default()
        });
        deps.len() - 1
    });
    &mut deps[index]
}

fn dep_table(name: &str) -> Option<DepTable> {
    match name {
        "dependencies" => Some(DepTable::Normal),
        "dev-dependencies" | "dev_dependencies" => Some(DepTable::Dev),
        "build-dependencies" | "build_dependencies" => Some(DepTable::Build),
        _ => None,
    }
}

fn classify_header(path: &[String]) -> Section {
    let (table_name, workspace, target_cfg, rest) = match path {
        [first, second, rest @ ..] if first == "workspace" && second == "dependencies" => {
            (second.as_str(), true, None, rest)
        }
        [first, cfg, table, rest @ ..] if first == "target" => {
            (table.as_str(), false, Some(cfg.clone()), rest)
        }
        [table, rest @ ..] => (table.as_str(), false, None, rest),
        [] => return Section::Other,
    };
    let Some(table) = dep_table(table_name) else {
        return Section::Other;
    };
    match rest {
        [] => Section::Table {
            workspace,
            table,
            target_cfg,
        },
        [key] => Section::Entry {
            key: key.clone(),
            workspace,
            table,
            target_cfg,
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

/// Quote and escape state for the TOML subset the scanner understands.
///
/// A `\` inside a basic string (`"…"`) escapes the next character, so `\"` does not end the
/// string and does not start a comment. Literal strings (`'…'`) have no escapes.
#[derive(Default)]
struct Lex {
    quote: Option<char>,
    escaped: bool,
}

impl Lex {
    /// Advance by `c`. Returns whether `c` sits outside a string.
    fn code(&mut self, c: char) -> bool {
        if self.escaped {
            self.escaped = false;
            return false;
        }
        if self.quote == Some('"') && c == '\\' {
            self.escaped = true;
            return false;
        }
        if let Some(quote) = self.quote {
            if c == quote {
                self.quote = None;
            }
            return false;
        }
        if c == '"' || c == '\'' {
            self.quote = Some(c);
            return false;
        }
        true
    }
}

/// The net `{`/`[` nesting a line opens, ignoring quoted text. A table header line
/// (`[dependencies]`) balances to zero.
fn bracket_balance(line: &str) -> i32 {
    let mut balance = 0;
    let mut lex = Lex::default();
    for c in line.chars() {
        if !lex.code(c) {
            continue;
        }
        match c {
            '{' | '[' => balance += 1,
            '}' | ']' => balance -= 1,
            _ => {}
        }
    }
    balance
}

/// Remove a `#` comment that is not inside a string.
fn strip_comment(line: &str) -> String {
    let mut lex = Lex::default();
    for (index, c) in line.char_indices() {
        if lex.code(c) && c == '#' {
            return line[..index].to_owned();
        }
    }
    line.to_owned()
}

/// Split `key = value` at the first `=` outside quotes.
fn split_key_value(statement: &str) -> Option<(&str, &str)> {
    let mut lex = Lex::default();
    for (index, c) in statement.char_indices() {
        if lex.code(c) && c == '=' {
            return Some((statement[..index].trim(), &statement[index + 1..]));
        }
    }
    None
}

/// Split at `separator` outside quotes and outside nested `{}`/`[]`/`()`.
fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut lex = Lex::default();
    let mut depth = 0i32;
    let mut start = 0;
    for (index, c) in text.char_indices() {
        if !lex.code(c) {
            continue;
        }
        match c {
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth -= 1,
            c if c == separator && depth == 0 => {
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
