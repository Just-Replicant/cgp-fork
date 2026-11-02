//! `cargo xtask migrate` rewrites 0.10 names to the 1.0 handler family.
//!
//! The table in this module is the source of truth for the migration guide. A
//! mechanical rule rewrites imports and paths. A manual rule is printed and left
//! in the file, because the replacement is not one type.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::ExitCode;
use std::{fs, io};

use proc_macro2::Span;
use syn::parse::{Parse, Parser};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    Attribute, Ident, Item, ItemUse, Macro, Path as SynPath, PathArguments, UseTree, Visibility,
};

const HANDLER: &[&[&str]] = &[&["prelude"], &["extra", "prelude"], &["extra", "handler"]];
const DISPATCH: &[&[&str]] = &[&["prelude"], &["extra", "prelude"], &["extra", "dispatch"]];
const FIELD_IMPL: &[&[&str]] = &[
    &["prelude"],
    &["core", "prelude"],
    &["core", "field", "impls"],
];
const FIELD_TRAIT: &[&[&str]] = &[
    &["prelude"],
    &["core", "prelude"],
    &["core", "field", "traits"],
];
const RUN: &[&[&str]] = &[&["extra", "run"]];

struct Rule {
    old: &'static str,
    modules: &'static [&'static [&'static str]],
    /// Empty when the codemod must not guess a type.
    new_path: &'static [&'static str],
    /// Type arguments appended when the path already has `append_after` arguments.
    append: &'static [&'static str],
    append_after: usize,
    note: &'static str,
}

const fn rule(
    old: &'static str,
    modules: &'static [&'static [&'static str]],
    new_path: &'static [&'static str],
    note: &'static str,
) -> Rule {
    Rule {
        old,
        modules,
        new_path,
        append: &[],
        append_after: 0,
        note,
    }
}

const fn with_args(
    old: &'static str,
    modules: &'static [&'static [&'static str]],
    new_path: &'static [&'static str],
    append: &'static [&'static str],
    append_after: usize,
    note: &'static str,
) -> Rule {
    Rule {
        old,
        modules,
        new_path,
        append,
        append_after,
        note,
    }
}

fn rules() -> &'static [Rule] {
    static RULES: &[Rule] = &[
        rule(
            "CanCompute",
            HANDLER,
            &["handler", "CanHandle"],
            "sync handler",
        ),
        rule(
            "Computer",
            HANDLER,
            &["handler", "Handler"],
            "provider trait",
        ),
        rule(
            "ComputerComponent",
            HANDLER,
            &["handler", "HandlerComponent"],
            "component",
        ),
        rule(
            "CanComputeRef",
            HANDLER,
            &["handler", "CanHandle"],
            "input is `&T`",
        ),
        rule(
            "ComputerRef",
            HANDLER,
            &["handler", "Handler"],
            "input is `&T`",
        ),
        rule(
            "ComputerRefComponent",
            HANDLER,
            &["handler", "HandlerComponent"],
            "input is `&T`",
        ),
        rule(
            "CanTryCompute",
            HANDLER,
            &["handler", "CanHandle"],
            "`Output = Result<…>`",
        ),
        rule(
            "TryComputer",
            HANDLER,
            &["handler", "Handler"],
            "`Output = Result<…>`",
        ),
        rule(
            "TryComputerComponent",
            HANDLER,
            &["handler", "HandlerComponent"],
            "`Output = Result<…>`",
        ),
        rule(
            "CanTryComputeRef",
            HANDLER,
            &["handler", "CanHandle"],
            "borrowed fallible handler",
        ),
        rule(
            "TryComputerRef",
            HANDLER,
            &["handler", "Handler"],
            "borrowed fallible handler",
        ),
        rule(
            "TryComputerRefComponent",
            HANDLER,
            &["handler", "HandlerComponent"],
            "borrowed fallible handler",
        ),
        rule(
            "CanComputeAsync",
            HANDLER,
            &["handler", "CanHandleAsync"],
            "async handler",
        ),
        rule(
            "AsyncComputer",
            HANDLER,
            &["handler", "AsyncHandler"],
            "async provider",
        ),
        rule(
            "AsyncComputerComponent",
            HANDLER,
            &["handler", "AsyncHandlerComponent"],
            "async component",
        ),
        rule(
            "CanComputeAsyncRef",
            HANDLER,
            &["handler", "CanHandleAsync"],
            "borrowed async handler",
        ),
        rule(
            "AsyncComputerRef",
            HANDLER,
            &["handler", "AsyncHandler"],
            "borrowed async provider",
        ),
        rule(
            "AsyncComputerRefComponent",
            HANDLER,
            &["handler", "AsyncHandlerComponent"],
            "borrowed async component",
        ),
        rule(
            "CanHandle",
            HANDLER,
            &["handler", "CanHandleAsync"],
            "the 0.10 async fallible consumer",
        ),
        rule(
            "Handler",
            HANDLER,
            &["handler", "AsyncHandler"],
            "the 0.10 async fallible provider; the new sync provider is `handler::Handler`",
        ),
        rule(
            "HandlerComponent",
            HANDLER,
            &["handler", "AsyncHandlerComponent"],
            "the 0.10 component; the new sync component is `handler::HandlerComponent`",
        ),
        rule(
            "CanHandleRef",
            HANDLER,
            &["handler", "CanHandleAsync"],
            "borrowed 0.10 handler",
        ),
        rule(
            "HandlerRef",
            HANDLER,
            &["handler", "AsyncHandler"],
            "borrowed 0.10 provider",
        ),
        rule(
            "HandlerRefComponent",
            HANDLER,
            &["handler", "AsyncHandlerComponent"],
            "borrowed 0.10 component",
        ),
        rule(
            "CanProduce",
            HANDLER,
            &["handler", "CanHandle"],
            "input is `()`",
        ),
        rule(
            "Producer",
            HANDLER,
            &["handler", "Handler"],
            "input is `()`",
        ),
        rule(
            "ProducerComponent",
            HANDLER,
            &["handler", "HandlerComponent"],
            "input is `()`",
        ),
        rule(
            "PromoteAsync",
            HANDLER,
            &["handler", "Promote"],
            "sync handler to async",
        ),
        rule(
            "PromoteRef",
            HANDLER,
            &["handler", "PromoteDeref"],
            "deref the input",
        ),
        rule(
            "TryPromote",
            HANDLER,
            &["handler", "PromoteOk"],
            "wrap the output in `Ok`",
        ),
        rule(
            "ReturnInput",
            HANDLER,
            &["handler", "ReturnInput"],
            "same role, new module",
        ),
        rule(
            "PromoteComputer",
            HANDLER,
            &["compat", "Compat"],
            "or wire `AsyncHandlerComponent: Promote<P>`",
        ),
        rule(
            "PromoteTryComputer",
            HANDLER,
            &["compat", "Compat"],
            "or wire the unified handler directly",
        ),
        rule(
            "PromoteProducer",
            HANDLER,
            &["compat", "Compat"],
            "or wire the unified handler directly",
        ),
        rule(
            "PromoteAsyncComputer",
            HANDLER,
            &["compat", "Compat"],
            "or wire the unified handler directly",
        ),
        rule(
            "PromoteHandler",
            HANDLER,
            &["compat", "Compat"],
            "or wire the unified handler directly",
        ),
        rule(
            "Promote",
            HANDLER,
            &[],
            "`IgnoreInput`, `PromoteOk`, or `Promote`, depending on which 0.10 promotion this was",
        ),
        rule(
            "PipeHandlers",
            HANDLER,
            &[],
            "`Pipe` or `TryPipe` when the handlers return `Result`",
        ),
        rule(
            "ComposeHandlers",
            HANDLER,
            &[],
            "`Pipe<Product![A, B]>` or `TryPipe`",
        ),
        rule(
            "MatchWithHandlers",
            DISPATCH,
            &["handler", "MatchWith"],
            "`All`, `Owned`",
        ),
        with_args(
            "MatchWithHandlersRef",
            DISPATCH,
            &["handler", "MatchWith"],
            &["handler::All", "handler::Ref"],
            1,
            "`MatchWith<H, All, Ref>`",
        ),
        with_args(
            "MatchWithHandlersMut",
            DISPATCH,
            &["handler", "MatchWith"],
            &["handler::All", "handler::Mut"],
            1,
            "`MatchWith<H, All, Mut>`",
        ),
        with_args(
            "MatchFirstWithHandlers",
            DISPATCH,
            &["handler", "MatchWith"],
            &["handler::First"],
            1,
            "`MatchWith<H, First>`",
        ),
        with_args(
            "MatchFirstWithHandlersRef",
            DISPATCH,
            &["handler", "MatchWith"],
            &["handler::First", "handler::Ref"],
            1,
            "`MatchWith<H, First, Ref>`",
        ),
        with_args(
            "MatchFirstWithHandlersMut",
            DISPATCH,
            &["handler", "MatchWith"],
            &["handler::First", "handler::Mut"],
            1,
            "`MatchWith<H, First, Mut>`",
        ),
        rule(
            "MatchWithFieldHandlers",
            DISPATCH,
            &["handler", "MatchFields"],
            "`All`, `Owned`",
        ),
        rule(
            "MatchWithValueHandlers",
            DISPATCH,
            &["handler", "MatchValues"],
            "`All`, `Owned`",
        ),
        with_args(
            "MatchWithFieldHandlersRef",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::All", "handler::Ref"],
            1,
            "`MatchFields<P, All, Ref>`",
        ),
        with_args(
            "MatchWithValueHandlersRef",
            DISPATCH,
            &["handler", "MatchValues"],
            &["handler::All", "handler::Ref"],
            1,
            "`MatchValues<P, All, Ref>`",
        ),
        with_args(
            "MatchWithValueHandlersMut",
            DISPATCH,
            &["handler", "MatchValues"],
            &["handler::All", "handler::Mut"],
            1,
            "`MatchValues<P, All, Mut>`",
        ),
        rule(
            "MatchWithFieldHandlersInputs",
            DISPATCH,
            &["handler", "MatchFields"],
            "same matcher; the input shape is the strategy",
        ),
        with_args(
            "MatchWithFieldHandlersInputsRef",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::All", "handler::Ref"],
            1,
            "`MatchFields<P, All, Ref>`",
        ),
        with_args(
            "MatchWithFieldHandlersInputsMut",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::All", "handler::Mut"],
            1,
            "`MatchFields<P, All, Mut>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlers",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First"],
            1,
            "`MatchFields<P, First>`",
        ),
        with_args(
            "MatchFirstWithValueHandlers",
            DISPATCH,
            &["handler", "MatchValues"],
            &["handler::First"],
            1,
            "`MatchValues<P, First>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlersRef",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First", "handler::Ref"],
            1,
            "`MatchFields<P, First, Ref>`",
        ),
        with_args(
            "MatchFirstWithValueHandlersRef",
            DISPATCH,
            &["handler", "MatchValues"],
            &["handler::First", "handler::Ref"],
            1,
            "`MatchValues<P, First, Ref>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlersMut",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First", "handler::Mut"],
            1,
            "`MatchFields<P, First, Mut>`",
        ),
        with_args(
            "MatchFirstWithValueHandlersMut",
            DISPATCH,
            &["handler", "MatchValues"],
            &["handler::First", "handler::Mut"],
            1,
            "`MatchValues<P, First, Mut>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlersInputs",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First"],
            1,
            "`MatchFields<P, First>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlersInputsRef",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First", "handler::Ref"],
            1,
            "`MatchFields<P, First, Ref>`",
        ),
        with_args(
            "MatchFirstWithFieldHandlersInputsMut",
            DISPATCH,
            &["handler", "MatchFields"],
            &["handler::First", "handler::Mut"],
            1,
            "`MatchFields<P, First, Mut>`",
        ),
        rule(
            "ExtractFieldAndHandle",
            DISPATCH,
            &["handler", "OnField"],
            "`OnField<Tag, P>`",
        ),
        with_args(
            "ExtractFirstFieldAndHandle",
            DISPATCH,
            &["handler", "OnField"],
            &["handler::First"],
            2,
            "`OnField<Tag, P, First>`",
        ),
        rule(
            "HandleFieldValue",
            DISPATCH,
            &["handler", "FieldValue"],
            "`FieldValue<P>`",
        ),
        with_args(
            "HandleFirstFieldValue",
            DISPATCH,
            &["handler", "FieldValue"],
            &["handler::First"],
            1,
            "`FieldValue<P, First>`",
        ),
        rule(
            "DowncastAndHandle",
            DISPATCH,
            &["handler", "Downcast"],
            "`Downcast<Inner, P>`",
        ),
        rule(
            "DispatchMatchers",
            DISPATCH,
            &["handler", "DispatchHandlers"],
            "tries `Result<_, Remainder>` handlers",
        ),
        rule(
            "BuildWithHandlers",
            DISPATCH,
            &[],
            "`BuildWith` or `TryBuildWith` when the handlers return `Result`",
        ),
        rule("BuildAndMerge", DISPATCH, &[], "`Merge` or `TryMerge`"),
        rule(
            "BuildAndMergeOutputs",
            DISPATCH,
            &[],
            "`MergeOutputs` or `TryMergeOutputs`",
        ),
        rule(
            "ToBuildAndMergeHandler",
            DISPATCH,
            &[],
            "`ToMerge` or `ToTryMerge`",
        ),
        rule(
            "BuildAndSetField",
            DISPATCH,
            &[],
            "`SetField` or `TrySetField`",
        ),
        rule(
            "HasFieldHandlers",
            DISPATCH,
            &[],
            "`VariantHandlers<P, S>`; the trait shape changed",
        ),
        rule(
            "MapFieldHandler",
            DISPATCH,
            &[],
            "`VariantHandlers<P, S>`; the trait shape changed",
        ),
        rule(
            "ToFieldHandlers",
            DISPATCH,
            &[],
            "`VariantHandlers<P, S>`; the trait shape changed",
        ),
        rule(
            "MapExtractFieldAndHandle",
            DISPATCH,
            &[],
            "`VariantHandlers<P, All>`",
        ),
        rule(
            "MapExtractFirstFieldAndHandle",
            DISPATCH,
            &[],
            "`VariantHandlers<P, First>`",
        ),
        rule(
            "IsRef",
            FIELD_IMPL,
            &["core", "field", "types", "Ref"],
            "access mode",
        ),
        rule(
            "IsMut",
            FIELD_IMPL,
            &["core", "field", "types", "Mut"],
            "access mode",
        ),
        rule(
            "IsOwned",
            &[&["core", "field", "impls"]],
            &["core", "field", "types", "Owned"],
            "access mode; not re-exported from the prelude",
        ),
        rule(
            "MapTypeRef",
            FIELD_TRAIT,
            &["core", "field", "traits", "Mode"],
            "`<M as Mode>::Map<'a, T>`",
        ),
        rule(
            "CanSendRun",
            RUN,
            &[],
            "put `#[cgp_async(Send)]` on the component that is spawned; `CanRun` stays",
        ),
        rule(
            "SendRunner",
            RUN,
            &[],
            "put `#[cgp_async(Send)]` on the component that is spawned",
        ),
        rule(
            "SendRunnerComponent",
            RUN,
            &[],
            "put `#[cgp_async(Send)]` on the component that is spawned",
        ),
    ];
    RULES
}

const ATTRIBUTES: &[(&str, &str, &str)] = &[
    (
        "cgp_computer",
        "cgp_handler",
        "same arguments; an async function becomes an `AsyncHandler`",
    ),
    (
        "cgp_producer",
        "cgp_handler",
        "a producer is a handler of `()`",
    ),
    (
        "derive_promote",
        "derive_handler",
        "forwards the component method to `Handler`",
    ),
    (
        "async_trait",
        "cgp_async",
        "the cgp-fork attribute, not `async_trait::async_trait`; `--keep-async-trait` leaves it",
    ),
];

/// Markdown table of every 0.10 name the guide and the codemod share.
pub fn migration_table() -> String {
    let mut out = String::from("| Old name | Replacement | Codemod |\n|---|---|---|\n");
    for rule in rules() {
        let replacement = if rule.new_path.is_empty() {
            format!("manual: {}", rule.note)
        } else if rule.append.is_empty() {
            format!("`cgp_fork::{}`", rule.new_path.join("::"))
        } else {
            format!("`cgp_fork::{}` ({})", rule.new_path.join("::"), rule.note)
        };
        let codemod = if rule.new_path.is_empty() {
            "left in place"
        } else if rule.append.is_empty() {
            "rewrites the path"
        } else {
            "rewrites the path and appends type arguments when they are all written out"
        };
        out.push_str(&format!("| `{}` | {replacement} | {codemod} |\n", rule.old));
    }
    for (old, new, note) in ATTRIBUTES {
        out.push_str(&format!(
            "| `#[{old}]` | `#[{new}]` | rewrites the attribute ({note}) |\n"
        ));
    }
    out.push_str(
        "\nMethod names (`compute`, `try_compute`, `produce`, `send_run`) are not rewritten. \
         The new methods are `handle` and `handle_async`.\n",
    );
    out
}

pub struct MigrateOptions {
    pub crates: Vec<String>,
    pub explicit_only: bool,
    pub keep_async_trait: bool,
}

impl Default for MigrateOptions {
    fn default() -> Self {
        Self {
            crates: vec!["cgp_fork".to_owned()],
            explicit_only: false,
            keep_async_trait: false,
        }
    }
}

pub struct MigrateResult {
    pub source: String,
    pub warnings: Vec<String>,
}

/// Rewrites one Rust file. The result uses `\n` line endings.
pub fn migrate_source(source: &str, options: &MigrateOptions) -> Result<MigrateResult, String> {
    let newline = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let source = source.replace("\r\n", "\n");
    let file = syn::parse_file(&source).map_err(|error| error.to_string())?;
    let mut edits = Edits::default();
    let mut warned = BTreeSet::new();
    rewrite_items(&file.items, &source, options, &mut edits, &mut warned);
    let mut rewritten = apply_edits(&source, edits.replacements);
    if newline == "\r\n" {
        rewritten = rewritten.replace('\n', "\r\n");
    }
    Ok(MigrateResult {
        source: rewritten,
        warnings: edits.warnings,
    })
}

pub fn migrate_command(args: Vec<String>) -> ExitCode {
    let mut options = MigrateOptions::default();
    options.crates.clear();
    let mut check = false;
    let mut guide = false;
    let mut paths = Vec::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--guide" => guide = true,
            "--explicit-only" => options.explicit_only = true,
            "--keep-async-trait" => options.keep_async_trait = true,
            "--crate" => {
                let Some(name) = args.next() else {
                    eprintln!("--crate needs the dependency name");
                    return ExitCode::from(2);
                };
                options.crates.push(name);
            }
            "--help" | "-h" => {
                crate::wiring::print_help();
                return ExitCode::SUCCESS;
            }
            other if other.starts_with('-') => {
                eprintln!("unknown option `{other}`");
                return ExitCode::from(2);
            }
            other => paths.push(other.to_owned()),
        }
    }
    if guide {
        println!("{}", migration_table());
        return ExitCode::SUCCESS;
    }
    if options.crates.is_empty() {
        options.crates.push("cgp_fork".to_owned());
    }
    if paths.is_empty() {
        paths.push(".".to_owned());
    }

    let mut files = Vec::new();
    for path in &paths {
        if let Err(error) = collect_rust(Path::new(path), &mut files) {
            eprintln!("{path}: {error}");
            return ExitCode::FAILURE;
        }
    }

    let mut changed = 0usize;
    for path in files {
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("{}: {error}", path.display());
                return ExitCode::FAILURE;
            }
        };
        let rewritten = match migrate_source(&source, &options) {
            Ok(rewritten) => rewritten,
            Err(error) => {
                eprintln!("{}: {error}", path.display());
                return ExitCode::FAILURE;
            }
        };
        for warning in &rewritten.warnings {
            eprintln!("{}: {warning}", path.display());
        }
        if rewritten.source != source {
            changed += 1;
            if !check && let Err(error) = fs::write(&path, rewritten.source) {
                eprintln!("{}: {error}", path.display());
                return ExitCode::FAILURE;
            }
        }
    }
    if check && changed > 0 {
        eprintln!("{changed} file(s) would change");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn collect_rust(path: &Path, files: &mut Vec<std::path::PathBuf>) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    if meta.is_dir() {
        for entry in fs::read_dir(path)? {
            let child = entry?.path();
            if child.is_dir() {
                let name = child
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if matches!(name, "target" | ".git" | "node_modules") {
                    continue;
                }
                collect_rust(&child, files)?;
            } else if child.extension().is_some_and(|ext| ext == "rs") {
                files.push(child);
            }
        }
        return Ok(());
    }
    files.push(path.to_path_buf());
    Ok(())
}

#[derive(Default)]
struct Edits {
    replacements: Vec<(usize, usize, String)>,
    warnings: Vec<String>,
}

impl Edits {
    fn replace(&mut self, source: &str, span: Span, text: String) {
        let Some((start, end)) = span_range(source, span) else {
            return;
        };
        if source.get(start..end) == Some(text.as_str()) {
            return;
        }
        self.replacements.push((start, end, text));
    }

    fn warn(&mut self, warned: &mut BTreeSet<String>, message: String) {
        if warned.insert(message.clone()) {
            self.warnings.push(message);
        }
    }
}

struct Scope {
    renames: BTreeMap<String, String>,
    modules: BTreeMap<String, (String, Vec<String>)>,
    globs: Vec<(&'static Rule, String)>,
}

fn rewrite_items(
    items: &[Item],
    source: &str,
    options: &MigrateOptions,
    edits: &mut Edits,
    warned: &mut BTreeSet<String>,
) {
    let scope = scope_for(items, source, options, edits, warned);
    let mut visitor = Rewriter {
        source,
        options,
        edits,
        warned,
        scope,
    };
    for item in items {
        if !matches!(item, Item::Use(_)) {
            visitor.visit_item(item);
        }
    }
}

fn scope_for(
    items: &[Item],
    source: &str,
    options: &MigrateOptions,
    edits: &mut Edits,
    warned: &mut BTreeSet<String>,
) -> Scope {
    let mut scope = Scope {
        renames: BTreeMap::new(),
        modules: BTreeMap::new(),
        globs: Vec::new(),
    };
    for item in items {
        if let Item::Use(item_use) = item {
            absorb_use(item_use, source, options, &mut scope, edits, warned);
        }
    }
    scope
}

struct Leaf {
    path: Vec<String>,
    alias: Option<String>,
    glob: bool,
}

fn absorb_use(
    item: &ItemUse,
    source: &str,
    options: &MigrateOptions,
    scope: &mut Scope,
    edits: &mut Edits,
    warned: &mut BTreeSet<String>,
) {
    let mut leaves = Vec::new();
    flatten_use(&[], &item.tree, &mut leaves);
    let mut changed = false;
    let mut rendered = Vec::new();
    let vis = visibility_prefix(&item.vis, source);

    for leaf in &leaves {
        let Some((crate_name, rest)) = split_crate(&leaf.path, options) else {
            rendered.push(render_leaf(&vis, leaf));
            continue;
        };
        if leaf.glob {
            if !options.explicit_only {
                for rule in rules() {
                    if rule.modules.iter().any(|module| *module == rest.as_slice()) {
                        scope.globs.push((rule, crate_name.clone()));
                    }
                }
            }
            rendered.push(render_leaf(&vis, leaf));
            continue;
        }
        let Some(name) = rest.last() else {
            rendered.push(render_leaf(&vis, leaf));
            continue;
        };
        let parent = &rest[..rest.len() - 1];
        if let Some(rule) = lookup(parent, name) {
            if rule.new_path.is_empty() {
                edits.warn(
                    warned,
                    format!("`{name}` is not rewritten automatically: {}", rule.note),
                );
                rendered.push(render_leaf(&vis, leaf));
                continue;
            }
            changed = true;
            let new_leaf = (*rule.new_path.last().unwrap_or(&name.as_str())).to_owned();
            let mut prefix = vec![crate_name];
            prefix.extend(
                rule.new_path[..rule.new_path.len() - 1]
                    .iter()
                    .map(|s| (*s).to_owned()),
            );
            if leaf.alias.is_none() && new_leaf != *name {
                scope.renames.insert(name.clone(), new_leaf.clone());
            }
            rendered.push(format!(
                "{vis}use {}{alias};",
                join_path(&prefix, &new_leaf),
                alias = leaf
                    .alias
                    .as_ref()
                    .map(|alias| format!(" as {alias}"))
                    .unwrap_or_default()
            ));
        } else if is_known_module(&rest) && leaf.alias.is_none() {
            scope
                .modules
                .insert(name.clone(), (crate_name, rest.clone()));
            rendered.push(render_leaf(&vis, leaf));
        } else {
            rendered.push(render_leaf(&vis, leaf));
        }
    }

    if changed {
        edits.replace(source, item.span(), rendered.join("\n"));
    }
}

fn render_leaf(vis: &str, leaf: &Leaf) -> String {
    if leaf.glob {
        return format!("{vis}use {}::*;", leaf.path.join("::"));
    }
    let name = leaf.path.last().cloned().unwrap_or_default();
    let prefix = leaf.path[..leaf.path.len() - 1].to_vec();
    let alias = leaf
        .alias
        .as_ref()
        .map(|alias| format!(" as {alias}"))
        .unwrap_or_default();
    format!("{vis}use {}{alias};", join_path(&prefix, &name))
}

fn join_path(prefix: &[String], leaf: &str) -> String {
    if prefix.is_empty() {
        leaf.to_owned()
    } else {
        format!("{}::{leaf}", prefix.join("::"))
    }
}

fn flatten_use(prefix: &[String], tree: &UseTree, out: &mut Vec<Leaf>) {
    match tree {
        UseTree::Path(path) => {
            let mut next = prefix.to_vec();
            next.push(path.ident.to_string());
            flatten_use(&next, &path.tree, out);
        }
        UseTree::Name(name) => out.push(Leaf {
            path: push_name(prefix, &name.ident),
            alias: None,
            glob: false,
        }),
        UseTree::Rename(name) => out.push(Leaf {
            path: push_name(prefix, &name.ident),
            alias: Some(name.rename.to_string()),
            glob: false,
        }),
        UseTree::Glob(_) => out.push(Leaf {
            path: prefix.to_vec(),
            alias: None,
            glob: true,
        }),
        UseTree::Group(group) => {
            for item in &group.items {
                flatten_use(prefix, item, out);
            }
        }
    }
}

fn push_name(prefix: &[String], ident: &Ident) -> Vec<String> {
    let mut path = prefix.to_vec();
    path.push(ident.to_string());
    path
}

fn visibility_prefix(vis: &Visibility, source: &str) -> String {
    match vis {
        Visibility::Inherited => String::new(),
        Visibility::Public(_) => "pub ".to_owned(),
        Visibility::Restricted(_) => restricted_prefix(vis, source),
    }
}

fn restricted_prefix(vis: &Visibility, source: &str) -> String {
    span_range(source, vis.span())
        .and_then(|(start, end)| source.get(start..end))
        .map_or_else(|| "pub ".to_owned(), |text| format!("{text} "))
}

struct Rewriter<'a> {
    source: &'a str,
    options: &'a MigrateOptions,
    edits: &'a mut Edits,
    warned: &'a mut BTreeSet<String>,
    scope: Scope,
}

impl Rewriter<'_> {
    fn render_path(&mut self, path: &SynPath) -> Option<String> {
        let names: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        if names.is_empty() {
            return None;
        }
        if let Some((crate_name, rest)) = split_crate(&names, self.options) {
            let name = rest.last()?;
            let parent = &rest[..rest.len() - 1];
            let rule = lookup(parent, name)?;
            return self.finish(rule, &crate_name, path);
        }
        if names.len() == 1 {
            return self.render_bare(&names[0], path);
        }
        if let Some((crate_name, module)) = self.scope.modules.get(&names[0]).cloned() {
            let mut parent = module;
            parent.extend(names[1..names.len() - 1].iter().cloned());
            let name = names.last()?;
            let rule = lookup(&parent, name)?;
            return self.finish(rule, &crate_name, path);
        }
        None
    }

    fn render_bare(&mut self, name: &str, path: &SynPath) -> Option<String> {
        if let Some(new_name) = self.scope.renames.get(name).cloned() {
            return Some(format!("{new_name}{}", angle_args(self.source, path)));
        }
        let (rule, crate_name) = self
            .scope
            .globs
            .iter()
            .find(|(rule, _)| rule.old == name)
            .map(|(rule, crate_name)| (*rule, crate_name.clone()))?;
        self.finish(rule, &crate_name, path)
    }

    fn finish(&mut self, rule: &Rule, crate_name: &str, path: &SynPath) -> Option<String> {
        if rule.new_path.is_empty() {
            self.edits.warn(
                self.warned,
                format!(
                    "`{}` is not rewritten automatically: {}",
                    rule.old, rule.note
                ),
            );
            return None;
        }
        let base = format!("{crate_name}::{}", rule.new_path.join("::"));
        let args = angle_args(self.source, path);
        if rule.append.is_empty() {
            return Some(format!("{base}{args}"));
        }
        let count = path
            .segments
            .last()
            .and_then(|segment| match &segment.arguments {
                PathArguments::AngleBracketed(args) => Some(args.args.len()),
                _ => None,
            })
            .unwrap_or(0);
        if args.is_empty() || count != rule.append_after {
            self.edits.warn(
                self.warned,
                format!(
                    "`{}` was renamed but its type arguments were left as written; expected {}",
                    rule.old, rule.note
                ),
            );
            return Some(format!("{base}{args}"));
        }
        let inner = args.trim();
        let inner = inner
            .strip_prefix('<')
            .and_then(|text| text.strip_suffix('>'))
            .unwrap_or(inner);
        let extra = rule
            .append
            .iter()
            .map(|part| format!("{crate_name}::{part}"))
            .collect::<Vec<_>>()
            .join(", ");
        let separator = if inner.trim().is_empty() { "" } else { ", " };
        Some(format!("{base}<{inner}{separator}{extra}>"))
    }

    fn token_text(&mut self, ident: &Ident) -> Option<String> {
        let name = ident.to_string();
        if let Some(new_name) = self.scope.renames.get(&name) {
            return Some(new_name.clone());
        }
        let (rule, crate_name) = self
            .scope
            .globs
            .iter()
            .find(|(rule, _)| rule.old == name)
            .map(|(rule, crate_name)| (*rule, crate_name.clone()))?;
        if rule.new_path.is_empty() || !rule.append.is_empty() {
            let message = if rule.new_path.is_empty() {
                format!("`{name}` is not rewritten automatically: {}", rule.note)
            } else {
                format!("`{name}` inside a macro was left in place; {}", rule.note)
            };
            self.edits.warn(self.warned, message);
            return None;
        }
        Some(format!("{crate_name}::{}", rule.new_path.join("::")))
    }

    fn walk_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        for token in tokens {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    if let Some(text) = self.token_text(&ident) {
                        self.edits.replace(self.source, ident.span(), text);
                    }
                }
                proc_macro2::TokenTree::Group(group) => self.walk_tokens(group.stream()),
                proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
            }
        }
    }
}

impl Visit<'_> for Rewriter<'_> {
    fn visit_item_use(&mut self, _item: &ItemUse) {}

    fn visit_item_mod(&mut self, item: &syn::ItemMod) {
        if let Some((_, items)) = &item.content {
            rewrite_items(items, self.source, self.options, self.edits, self.warned);
        }
    }

    fn visit_attribute(&mut self, attr: &Attribute) {
        if let Some(new_name) = attribute_name(attr.path(), self.options)
            && let Some(ident) = attr.path().segments.last()
        {
            self.edits
                .replace(self.source, ident.ident.span(), new_name.to_owned());
        }
        if let syn::Meta::List(list) = &attr.meta
            && let Ok(path) = Parser::parse2(SynPath::parse, list.tokens.clone())
        {
            self.visit_path(&path);
        }
    }

    fn visit_path(&mut self, path: &SynPath) {
        if let Some(text) = self.render_path(path) {
            self.edits.replace(self.source, path.span(), text);
            return;
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_macro(&mut self, mac: &Macro) {
        let name = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default();
        if matches!(
            name.as_str(),
            "delegate_components"
                | "delegate_and_check_components"
                | "check_components"
                | "cgp_preset"
        ) {
            self.walk_tokens(mac.tokens.clone());
        }
    }
}

fn attribute_name(path: &SynPath, options: &MigrateOptions) -> Option<&'static str> {
    if path.segments.len() != 1 {
        return None;
    }
    let name = path.segments[0].ident.to_string();
    if options.keep_async_trait && name == "async_trait" {
        return None;
    }
    ATTRIBUTES
        .iter()
        .find(|(old, _, _)| *old == name)
        .map(|(_, new, _)| *new)
}

fn lookup(parent: &[String], name: &str) -> Option<&'static Rule> {
    rules()
        .iter()
        .find(|rule| rule.old == name && rule.modules.iter().any(|module| *module == parent))
}

fn is_known_module(path: &[String]) -> bool {
    rules()
        .iter()
        .any(|rule| rule.modules.iter().any(|module| *module == path))
}

fn split_crate(path: &[String], options: &MigrateOptions) -> Option<(String, Vec<String>)> {
    let first = path.first()?;
    options
        .crates
        .iter()
        .find(|name| *name == first)
        .map(|name| (name.clone(), path[1..].to_vec()))
}

fn angle_args(source: &str, path: &SynPath) -> String {
    let Some(segment) = path.segments.last() else {
        return String::new();
    };
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return String::new();
    };
    span_range(source, args.span())
        .and_then(|(start, end)| source.get(start..end))
        .unwrap_or("")
        .to_owned()
}

fn span_range(source: &str, span: Span) -> Option<(usize, usize)> {
    let start = offset(source, span.start().line, span.start().column)?;
    let end = offset(source, span.end().line, span.end().column)?;
    (start < end).then_some((start, end))
}

fn offset(source: &str, line: usize, column: usize) -> Option<usize> {
    if line == 0 {
        return None;
    }
    if line == 1 {
        return Some(column.min(source.len()));
    }
    let mut current = 1usize;
    for (at, ch) in source.char_indices() {
        if ch == '\n' {
            current += 1;
            if current == line {
                return Some((at + ch.len_utf8() + column).min(source.len()));
            }
        }
    }
    None
}

fn apply_edits(source: &str, mut edits: Vec<(usize, usize, String)>) -> String {
    edits.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    let mut out = source.to_owned();
    let mut floor = source.len();
    for (start, end, text) in edits {
        if end > floor || start > end || end > out.len() {
            continue;
        }
        out.replace_range(start..end, &text);
        floor = start;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{MigrateOptions, migrate_source, migration_table};

    #[test]
    fn book_embeds_the_migration_table() {
        let book = include_str!("../../book/src/migration.md");
        let start = "<!-- migration-table:start -->\n";
        let end = "<!-- migration-table:end -->";
        let table_at = book.find(start).expect("start marker");
        let rest = &book[table_at + start.len()..];
        let end_at = rest.find(end).expect("end marker");
        assert_eq!(&rest[..end_at], migration_table().as_str());
    }

    fn migrated(source: &str) -> String {
        migrate_source(source, &MigrateOptions::default())
            .unwrap()
            .source
    }

    #[test]
    fn rewrites_an_import_and_its_uses() {
        let source = "use cgp_fork::extra::handler::Computer;\n\nfn f(value: Computer) -> Computer { value }\n";
        let output = migrated(source);
        assert!(
            output.contains("use cgp_fork::handler::Handler;"),
            "{output}"
        );
        assert!(
            output.contains("fn f(value: Handler) -> Handler"),
            "{output}"
        );
        assert_eq!(migrated(&output), output);
    }

    #[test]
    fn rewrites_absolute_paths_and_leaves_the_new_module() {
        let source = "fn old(value: cgp_fork::extra::handler::Computer) {}\nfn now(value: cgp_fork::handler::Handler) {}\n";
        let output = migrated(source);
        assert!(
            output.contains("fn old(value: cgp_fork::handler::Handler)"),
            "{output}"
        );
        assert!(
            output.contains("fn now(value: cgp_fork::handler::Handler)"),
            "{output}"
        );
    }

    #[test]
    fn rewrites_attributes_but_not_the_async_trait_crate() {
        let source = "#[cgp_computer]\n#[cgp_producer]\n#[derive_promote(Foo)]\n#[async_trait]\n#[async_trait::async_trait]\nfn f() {}\n";
        let output = migrated(source);
        assert!(output.contains("#[cgp_handler]\n#[cgp_handler]\n#[derive_handler(Foo)]\n#[cgp_async]\n#[async_trait::async_trait]"), "{output}");
    }

    #[test]
    fn appends_match_type_arguments() {
        let source = "fn f(value: cgp_fork::extra::dispatch::MatchWithHandlersRef<H>) {}\n";
        let output = migrated(source);
        assert!(
            output.contains(
                "cgp_fork::handler::MatchWith<H, cgp_fork::handler::All, cgp_fork::handler::Ref>"
            ),
            "{output}"
        );
    }

    #[test]
    fn leaves_promote_and_warns() {
        let source = "use cgp_fork::extra::handler::Promote;\n";
        let result = migrate_source(source, &MigrateOptions::default()).unwrap();
        assert_eq!(result.source, source);
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.contains("Promote"))
        );
    }

    #[test]
    fn glob_rewrites_a_wiring_key() {
        let source = "use cgp_fork::prelude::*;\n\ndelegate_components! {\n    App { ComputerComponent: Hello }\n}\n";
        let output = migrated(source);
        assert!(
            output.contains("cgp_fork::handler::HandlerComponent: Hello"),
            "{output}"
        );
        assert!(!output.contains("ComputerComponent:"), "{output}");
    }

    #[test]
    fn explicit_only_keeps_glob_names() {
        let source = "use cgp_fork::prelude::*;\n\nfn f(value: Computer) {}\n";
        let options = MigrateOptions {
            explicit_only: true,
            ..MigrateOptions::default()
        };
        let output = migrate_source(source, &options).unwrap().source;
        assert!(output.contains("fn f(value: Computer)"), "{output}");
    }

    #[test]
    fn renamed_crate_is_honored() {
        let source = "use cgp::extra::handler::Producer;\n";
        let options = MigrateOptions {
            crates: vec!["cgp".to_owned()],
            ..MigrateOptions::default()
        };
        let output = migrate_source(source, &options).unwrap().source;
        assert!(output.contains("use cgp::handler::Handler;"), "{output}");
    }

    #[test]
    fn the_guide_lists_every_rule_and_attribute() {
        let table = migration_table();
        assert!(table.contains("`Computer`"));
        assert!(table.contains("`Promote`"));
        assert!(table.contains("`#[async_trait]`"));
        assert!(table.contains("handle_async"));
    }
}
