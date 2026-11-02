//! Wiring documents and the graph rendered from them.
//!
//! `delegate_components!` emits the same JSON this module builds, behind the `inspect`
//! feature. `cargo xtask wiring` reads the source with the macro parsers and prints
//! that JSON, or a Mermaid or DOT graph of context → component → provider.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::visit::Visit;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{
    Attribute, Ident, ItemImpl, Macro, Type, TypeParamBound, WherePredicate,
};

use crate::macro_core::types::cgp_preset::ItemCgpPreset;
use crate::macro_core::types::delegate_component::{
    DelegateMapping, DelegateTable, EvalDelegateEntries, cfg_attribute,
};

/// One component wired to a provider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WiringEdge {
    /// The component key, as written.
    pub component: String,
    /// The provider, as written.
    pub provider: String,
    /// Cfg predicates from `#[cfg_wiring(...)]` on the entry.
    pub cfgs: Vec<String>,
    /// The parent an `override` replaces. A bare `override` with one parent records that parent.
    pub override_of: Option<String>,
}

/// A `delegate_components!` table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WiringTableDoc {
    /// The context or table type.
    pub context: String,
    /// The table's generic parameters, empty when it has none.
    pub generics: String,
    /// `#[unchecked]` was written on the table.
    pub unchecked: bool,
    /// Cfg predicates from `#[cfg_wiring(...)]` on the table.
    pub cfgs: Vec<String>,
    /// Explicit component entries. Preset entries are listed in [`Self::presets`].
    pub entries: Vec<WiringEdge>,
    /// Preset paths replayed by this table.
    pub presets: Vec<String>,
}

/// A `cgp_preset!` bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresetDoc {
    /// The preset's name.
    pub name: String,
    /// `#[version(N)]`, when written.
    pub version: Option<u64>,
    /// Parent presets, as written.
    pub parents: Vec<String>,
    /// The preset's own entries, including overrides.
    pub entries: Vec<WiringEdge>,
}

/// A provider impl and the context bounds it requires.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderDoc {
    /// The provider type, as written.
    pub provider: String,
    /// Trait bounds on the context (`Self` for `#[cgp_impl]`, the first type
    /// parameter for `#[cgp_provider]`). Marker and formatting bounds are left out.
    pub dependencies: Vec<String>,
}

/// Every wiring table and preset found in one file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WiringFile {
    /// Path the scanner was given. Empty when the source was parsed on its own.
    pub path: String,
    /// `delegate_components!` and `delegate_and_check_components!` tables.
    pub tables: Vec<WiringTableDoc>,
    /// `cgp_preset!` bundles.
    pub presets: Vec<PresetDoc>,
    /// `#[cgp_provider]` and `#[cgp_impl]` providers.
    pub providers: Vec<ProviderDoc>,
}

/// How [`render`] prints a set of files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphFormat {
    /// A JSON array of the tables and presets.
    Json,
    /// A Mermaid `flowchart`.
    Mermaid,
    /// A Graphviz `digraph`.
    Dot,
}

impl WiringTableDoc {
    /// The JSON object stored in the hidden wiring const.
    pub fn to_json(&self) -> String {
        let mut json = String::from("{\"kind\":\"context\"");
        push_field(&mut json, "context", &self.context);
        push_field(&mut json, "generics", &self.generics);
        push_bool(&mut json, "unchecked", self.unchecked);
        push_array(&mut json, "cfgs", &self.cfgs);
        push_edges(&mut json, &self.entries);
        push_array(&mut json, "presets", &self.presets);
        json.push('}');
        json
    }
}

impl PresetDoc {
    /// The JSON object stored in the hidden preset const.
    pub fn to_json(&self) -> String {
        let mut json = String::from("{\"kind\":\"preset\"");
        push_field(&mut json, "name", &self.name);
        match self.version {
            Some(version) => {
                let _ = write!(json, ",\"version\":{version}");
            }
            None => json.push_str(",\"version\":null"),
        }
        push_array(&mut json, "parents", &self.parents);
        push_edges(&mut json, &self.entries);
        json.push('}');
        json
    }
}

/// Builds the document for one parsed table.
pub fn table_doc(table: &DelegateTable) -> syn::Result<WiringTableDoc> {
    let (unchecked, cfgs) = table_meta(&table.attributes);

    let mut entries = Vec::new();
    let mut presets = Vec::new();

    for statement in &table.entries.statements {
        for evaluated in statement.eval_entries(&table.table_type)? {
            entries.push(WiringEdge {
                component: tokens(&evaluated.key),
                provider: tokens(&evaluated.value),
                cfgs: evaluated.cfgs.iter().map(cfg_predicate).collect(),
                override_of: None,
            });
        }
    }

    for mapping in &table.entries.entries {
        push_mapping(mapping, &table.table_type, None, &mut entries, &mut presets)?;
    }

    Ok(WiringTableDoc {
        context: tokens(&table.table_type),
        generics: tokens(&table.impl_generics.generics),
        unchecked,
        cfgs,
        entries,
        presets,
    })
}

/// Builds the document for one parsed preset.
pub fn preset_doc(preset: &ItemCgpPreset) -> syn::Result<PresetDoc> {
    let provider: Type = syn::parse_quote!(());
    let mut entries = Vec::new();
    let mut presets = Vec::new();

    for mapping in &preset.entries.entries {
        // A bare `override` means the only parent, the same rule `cgp_preset!` checks.
        let inferred = match mapping {
            DelegateMapping::Override(entry) if entry.parent.is_none() => {
                match preset.parents.as_slice() {
                    [only] => Some(tokens(only)),
                    _ => None,
                }
            }
            _ => None,
        };
        push_mapping(mapping, &provider, inferred, &mut entries, &mut presets)?;
    }

    Ok(PresetDoc {
        name: preset.name.to_string(),
        version: preset
            .version
            .as_ref()
            .and_then(|lit| lit.base10_parse().ok()),
        parents: preset.parents.iter().map(tokens).collect(),
        entries,
    })
}

/// The hidden const `delegate_components!` emits. The facade's `__cgp_inspect!`
/// keeps it only when the `inspect` feature is on.
pub fn table_inspect_item(table: &DelegateTable) -> syn::Result<TokenStream> {
    let doc = table_doc(table)?;
    let json = doc.to_json();
    let ident = inspect_ident("WIRING", &doc.context, &json);
    Ok(inspect_macro(&ident, &json))
}

/// The hidden const `cgp_preset!` emits.
pub fn preset_inspect_item(preset: &ItemCgpPreset) -> syn::Result<TokenStream> {
    let doc = preset_doc(preset)?;
    let json = doc.to_json();
    let ident = inspect_ident("PRESET", &doc.name, &json);
    Ok(inspect_macro(&ident, &json))
}

/// Parses every wiring macro in `source`.
pub fn scan_source(source: &str) -> syn::Result<WiringFile> {
    let file = syn::parse_file(source)?;
    let mut found = SourceFinder::default();
    found.visit_file(&file);

    let mut wiring = WiringFile {
        providers: found.providers,
        ..WiringFile::default()
    };

    for mac in found.macros {
        match mac.name.as_str() {
            "delegate_components" | "delegate_and_check_components" => {
                let table: DelegateTable = syn::parse2(mac.tokens)?;
                wiring.tables.push(table_doc(&table)?);
            }
            "cgp_preset" => {
                let preset: ItemCgpPreset = syn::parse2(mac.tokens)?;
                wiring.presets.push(preset_doc(&preset)?);
            }
            _ => {}
        }
    }

    Ok(wiring)
}

/// Renders every file in `format`.
pub fn render(files: &[WiringFile], format: GraphFormat) -> String {
    match format {
        GraphFormat::Json => render_json(files),
        GraphFormat::Mermaid => render_mermaid(files),
        GraphFormat::Dot => render_dot(files),
    }
}

fn push_mapping(
    mapping: &DelegateMapping,
    table_type: &Type,
    override_of: Option<String>,
    entries: &mut Vec<WiringEdge>,
    presets: &mut Vec<String>,
) -> syn::Result<()> {
    match mapping {
        DelegateMapping::Preset(preset) => {
            presets.push(tokens(&preset.path));
            Ok(())
        }
        DelegateMapping::Override(entry) => {
            let parent = entry.parent.as_ref().map(tokens).or(override_of);
            push_mapping(&entry.entry, table_type, parent, entries, presets)
        }
        other => {
            for evaluated in other.eval_entries(table_type)? {
                entries.push(WiringEdge {
                    component: tokens(&evaluated.key),
                    provider: tokens(&evaluated.value),
                    cfgs: evaluated.cfgs.iter().map(cfg_predicate).collect(),
                    override_of: override_of.clone(),
                });
            }
            Ok(())
        }
    }
}

fn inspect_macro(ident: &Ident, json: &str) -> TokenStream {
    let root = crate::crate_path::crate_root();

    quote! {
        #root::__cgp_inspect! {
            #[allow(dead_code, non_upper_case_globals)]
            const #ident: &'static str = #json;
        }
    }
}

fn inspect_ident(kind: &str, name: &str, json: &str) -> Ident {
    let leaf = name
        .rsplit("::")
        .next()
        .unwrap_or(name)
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect::<String>();
    let leaf = if leaf.is_empty() {
        "Table".to_owned()
    } else {
        leaf
    };

    Ident::new(
        &format!("__CGP_{kind}_{leaf}_{:08x}", fnv1a(json.as_bytes())),
        Span::call_site(),
    )
}

fn fnv1a(bytes: &[u8]) -> u32 {
    let mut hash = 0x811c_9dc5_u32;

    for byte in bytes {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }

    hash
}

fn tokens(value: &impl ToTokens) -> String {
    let raw = value.to_token_stream().to_string();
    let mut out = String::new();
    let mut pending_space = false;

    for ch in raw.chars() {
        if ch.is_whitespace() {
            if !out.is_empty() {
                pending_space = true;
            }
        } else {
            if pending_space {
                out.push(' ');
                pending_space = false;
            }
            out.push(ch);
        }
    }

    out
}

fn cfg_predicate(attribute: &Attribute) -> String {
    match &attribute.meta {
        syn::Meta::List(list) => list.tokens.to_string(),
        _ => tokens(attribute),
    }
}

fn push_field(json: &mut String, name: &str, value: &str) {
    let _ = write!(json, ",\"{name}\":{}", json_string(value));
}

fn push_bool(json: &mut String, name: &str, value: bool) {
    let _ = write!(json, ",\"{name}\":{}", if value { "true" } else { "false" });
}

fn push_array(json: &mut String, name: &str, values: &[String]) {
    json.push_str(",\"");
    json.push_str(name);
    json.push_str("\":[");

    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }
        json.push_str(&json_string(value));
    }

    json.push(']');
}

fn push_edges(json: &mut String, edges: &[WiringEdge]) {
    json.push_str(",\"entries\":[");

    for (index, edge) in edges.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }

        json.push_str("{\"component\":");
        json.push_str(&json_string(&edge.component));
        json.push_str(",\"provider\":");
        json.push_str(&json_string(&edge.provider));
        push_array(json, "cfgs", &edge.cfgs);

        match &edge.override_of {
            Some(parent) => {
                json.push_str(",\"override\":");
                json.push_str(&json_string(parent));
            }
            None => json.push_str(",\"override\":null"),
        }

        json.push('}');
    }

    json.push(']');
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");

    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other if other.is_control() => {
                let _ = write!(out, "\\u{:04x}", other as u32);
            }
            other => out.push(other),
        }
    }

    out.push('"');
    out
}

fn render_json(files: &[WiringFile]) -> String {
    let mut json = String::from("{\"files\":[");

    for (index, file) in files.iter().enumerate() {
        if index > 0 {
            json.push(',');
        }

        json.push_str("{\"path\":");
        json.push_str(&json_string(&file.path));
        json.push_str(",\"tables\":[");

        for (index, table) in file.tables.iter().enumerate() {
            if index > 0 {
                json.push(',');
            }
            json.push_str(&table.to_json());
        }

        json.push_str("],\"presets\":[");

        for (index, preset) in file.presets.iter().enumerate() {
            if index > 0 {
                json.push(',');
            }
            json.push_str(&preset.to_json());
        }

        json.push_str("],\"providers\":[");

        for (index, provider) in file.providers.iter().enumerate() {
            if index > 0 {
                json.push(',');
            }
            json.push_str("{\"provider\":");
            json.push_str(&json_string(&provider.provider));
            push_array(&mut json, "dependencies", &provider.dependencies);
            json.push('}');
        }

        json.push_str("]}");
    }

    json.push_str("]}");
    json
}

struct GraphEdge {
    from: String,
    to: String,
    label: String,
}

fn render_mermaid(files: &[WiringFile]) -> String {
    let edges = graph_edges(files);
    let mut nodes = BTreeMap::new();
    let mut mermaid = String::from("flowchart LR\n");

    for edge in &edges {
        let from = node_id(&mut nodes, &edge.from);
        let to = node_id(&mut nodes, &edge.to);
        let _ = writeln!(mermaid, "  {from} -->|{}| {to}", mermaid_edge(&edge.label));
    }

    for (label, id) in &nodes {
        let _ = writeln!(mermaid, "  {id}[\"{}\"]", mermaid_node(label));
    }

    mermaid
}

fn render_dot(files: &[WiringFile]) -> String {
    let edges = graph_edges(files);
    let mut dot = String::from("digraph cgp_wiring {\n");

    for edge in edges {
        let _ = writeln!(
            dot,
            "  \"{}\" -> \"{}\" [label=\"{}\"];",
            dot_escape(&edge.from),
            dot_escape(&edge.to),
            dot_escape(&edge.label)
        );
    }

    dot.push_str("}\n");
    dot
}

fn graph_edges(files: &[WiringFile]) -> Vec<GraphEdge> {
    let mut presets: BTreeMap<String, Vec<&PresetDoc>> = BTreeMap::new();

    for file in files {
        for preset in &file.presets {
            presets
                .entry(preset.name.clone())
                .or_default()
                .push(preset);
        }
    }

    let mut edges = BTreeSet::new();

    for file in files {
        for table in &file.tables {
            for entry in &table.entries {
                link_entry(&mut edges, &table.context, entry);
            }

            for preset in &table.presets {
                let mut stack = Vec::new();
                let flat = flatten_preset(preset, &presets, &mut stack);

                if flat.is_empty() {
                    add_edge(
                        &mut edges,
                        table.context.clone(),
                        format!("{preset}::Provider"),
                        "preset",
                    );
                } else {
                    for entry in flat {
                        link_entry(&mut edges, &table.context, &entry);
                    }
                }
            }
        }

        for preset in &file.presets {
            let from = format!("{}::Provider", preset.name);

            for parent in &preset.parents {
                let parent_leaf = parent.rsplit("::").next().unwrap_or(parent);
                add_edge(
                    &mut edges,
                    from.clone(),
                    format!("{parent_leaf}::Provider"),
                    "parent",
                );
            }

            for entry in &preset.entries {
                link_entry(&mut edges, &from, entry);
            }
        }

        for provider in &file.providers {
            for dependency in &provider.dependencies {
                add_edge(
                    &mut edges,
                    provider.provider.clone(),
                    dependency.clone(),
                    "needs",
                );
            }
        }
    }

    edges
        .into_iter()
        .map(|(from, to, label)| GraphEdge { from, to, label })
        .collect()
}

fn link_entry(edges: &mut BTreeSet<(String, String, String)>, from: &str, entry: &WiringEdge) {
    add_edge(edges, from.to_owned(), entry.component.clone(), wires_label(&entry.cfgs));
    add_edge(
        edges,
        entry.component.clone(),
        entry.provider.clone(),
        provides_label(entry),
    );
}

fn add_edge(
    edges: &mut BTreeSet<(String, String, String)>,
    from: String,
    to: String,
    label: impl Into<String>,
) {
    edges.insert((from, to, label.into()));
}

fn wires_label(cfgs: &[String]) -> String {
    if cfgs.is_empty() {
        "wires".to_owned()
    } else {
        cfgs.join(", ")
    }
}

fn provides_label(entry: &WiringEdge) -> String {
    let mut label = String::from("provides");

    if let Some(parent) = &entry.override_of {
        let _ = write!(label, " override {parent}");
    }

    if !entry.cfgs.is_empty() {
        let _ = write!(label, " [{}]", entry.cfgs.join(", "));
    }

    label
}

/// Parent entries first, then the preset's own entries replace the same component.
fn flatten_preset<'a>(
    path: &str,
    presets: &'a BTreeMap<String, Vec<&'a PresetDoc>>,
    stack: &mut Vec<String>,
) -> Vec<WiringEdge> {
    let leaf = path.rsplit("::").next().unwrap_or(path).to_owned();

    if stack.iter().any(|seen| seen == &leaf) {
        return Vec::new();
    }

    let Some(preset) = resolve_preset(path, presets) else {
        return Vec::new();
    };

    stack.push(leaf);
    let mut entries = BTreeMap::<String, WiringEdge>::new();

    for parent in &preset.parents {
        for entry in flatten_preset(parent, presets, stack) {
            entries.insert(entry.component.clone(), entry);
        }
    }

    for entry in &preset.entries {
        entries.insert(entry.component.clone(), entry.clone());
    }

    stack.pop();
    entries.into_values().collect()
}

fn resolve_preset<'a>(
    path: &str,
    presets: &'a BTreeMap<String, Vec<&'a PresetDoc>>,
) -> Option<&'a PresetDoc> {
    if let Some(found) = presets.get(path).filter(|found| found.len() == 1) {
        return found.first().copied();
    }

    let leaf = path.rsplit("::").next().unwrap_or(path);
    presets
        .get(leaf)
        .filter(|found| found.len() == 1)
        .and_then(|found| found.first().copied())
}

fn node_id(nodes: &mut BTreeMap<String, String>, label: &str) -> String {
    if let Some(id) = nodes.get(label) {
        return id.clone();
    }

    let id = format!("n{}", nodes.len());
    nodes.insert(label.to_owned(), id.clone());
    id
}

fn mermaid_edge(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "#quot;")
        .replace('|', "/")
        .replace(['\n', '\r'], " ")
}

fn mermaid_node(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "#quot;")
        .replace(['\n', '\r'], " ")
        .replace(['[', ']'], "")
}

fn dot_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "")
}

#[derive(Default)]
struct SourceFinder {
    macros: Vec<FoundMacro>,
    providers: Vec<ProviderDoc>,
}

struct FoundMacro {
    name: String,
    tokens: TokenStream,
}

impl Visit<'_> for SourceFinder {
    fn visit_macro(&mut self, mac: &Macro) {
        let Some(name) = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
        else {
            return;
        };

        if matches!(
            name.as_str(),
            "delegate_components" | "delegate_and_check_components" | "cgp_preset"
        ) {
            self.macros.push(FoundMacro {
                name,
                tokens: mac.tokens.clone(),
            });
        }

        syn::visit::visit_macro(self, mac);
    }

    fn visit_item_impl(&mut self, item: &ItemImpl) {
        if let Some(provider) = provider_doc(item) {
            self.providers.push(provider);
        }

        syn::visit::visit_item_impl(self, item);
    }
}

fn provider_doc(item: &ItemImpl) -> Option<ProviderDoc> {
    let cgp_impl = item
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("cgp_impl"));
    let cgp_provider = item.attrs.iter().any(|attribute| {
        attribute.path().is_ident("cgp_provider") || attribute.path().is_ident("cgp_new_provider")
    });

    let provider = if let Some(attribute) = cgp_impl {
        cgp_impl_name(attribute)?
    } else if cgp_provider {
        tokens(&*item.self_ty)
    } else {
        return None;
    };

    let self_is_context = cgp_impl.is_some();
    let context = if self_is_context {
        None
    } else {
        item.generics
            .type_params()
            .next()
            .map(|param| param.ident.to_string())
    };

    Some(ProviderDoc {
        provider,
        dependencies: context_bounds(item, context.as_deref(), self_is_context),
    })
}

fn cgp_impl_name(attribute: &Attribute) -> Option<String> {
    let args: crate::macro_core::types::cgp_impl::ImplArgs = attribute.parse_args().ok()?;
    Some(tokens(&args.provider_type))
}

fn context_bounds(item: &ItemImpl, context: Option<&str>, self_is_context: bool) -> Vec<String> {
    let mut deps = Vec::new();

    if let Some(where_clause) = &item.generics.where_clause {
        for predicate in &where_clause.predicates {
            if let WherePredicate::Type(predicate) = predicate
                && is_context(&predicate.bounded_ty, context, self_is_context)
            {
                push_bounds(&predicate.bounds, &mut deps);
            }
        }
    }

    for param in item.generics.type_params() {
        if context.is_some_and(|name| param.ident == name) {
            push_bounds(&param.bounds, &mut deps);
        }
    }

    deps
}

fn push_bounds(bounds: &Punctuated<TypeParamBound, Plus>, deps: &mut Vec<String>) {
    for bound in bounds {
        let TypeParamBound::Trait(bound) = bound else {
            continue;
        };

        if bound.maybe.is_some() {
            continue;
        }

        let name = tokens(&bound.path);

        if !is_marker(&name) && !deps.iter().any(|existing| existing == &name) {
            deps.push(name);
        }
    }
}

fn is_context(ty: &Type, context: Option<&str>, self_is_context: bool) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };

    if path.qself.is_some() || path.path.segments.len() != 1 {
        return false;
    }

    let ident = path.path.segments[0].ident.to_string();
    (self_is_context && ident == "Self") || context.is_some_and(|name| ident == name)
}

fn is_marker(name: &str) -> bool {
    let leaf = name.rsplit("::").next().unwrap_or(name).trim();
    matches!(
        leaf,
        "Sized"
            | "Send"
            | "Sync"
            | "Unpin"
            | "Copy"
            | "Clone"
            | "Debug"
            | "Display"
            | "Default"
            | "Eq"
            | "PartialEq"
            | "Ord"
            | "PartialOrd"
            | "Hash"
            | "Format"
            | "AsRef"
            | "AsMut"
            | "Deref"
            | "Borrow"
            | "ToString"
            | "ToOwned"
            | "Iterator"
            | "IntoIterator"
            | "Future"
    )
}

fn table_meta(attributes: &[Attribute]) -> (bool, Vec<String>) {
    let mut unchecked = false;
    let mut cfgs = Vec::new();

    for attribute in attributes {
        if attribute.path().is_ident("unchecked") {
            unchecked = true;
        } else if attribute.path().is_ident("cfg_wiring")
            && let Ok(cfg) = cfg_attribute(attribute)
        {
            cfgs.push(cfg_predicate(&cfg));
        }
    }

    (unchecked, cfgs)
}

#[cfg(test)]
mod tests {
    use super::{GraphFormat, render, scan_source};

    #[test]
    fn render_reads_checks_profiles_and_preset_overrides() {
        let source = r#"
            #[cgp_impl(new GreetHello)]
            impl Greeter where Self: HasName + Clone {}

            fn demo() {
                delegate_components! {
                    #[unchecked]
                    #[cfg_wiring(test)]
                    App {
                        #[cfg_wiring(feature = "mock")]
                        GreeterComponent: GreetHello,
                        preset LoudPreset,
                    }
                }

                delegate_and_check_components! {
                    #[check_trait(CanUseChecked)]
                    Checked {
                        GreeterComponent: GreetHello,
                    }
                }

                cgp_preset! {
                    #[version(2)]
                    LoudPreset: Base {
                        override(Base) GreeterComponent: Loud,
                    }
                }

                cgp_preset! {
                    #[version(3)]
                    BarePreset: Base {
                        override GreeterComponent: Loud,
                    }
                }
            }
        "#;

        let file = scan_source(source).unwrap_or_else(|error| panic!("{error}"));
        let json = render(std::slice::from_ref(&file), GraphFormat::Json);

        assert!(json.contains("\"unchecked\":true"), "{json}");
        assert!(json.contains("\"cfgs\":[\"test\"]"), "{json}");
        assert!(!json.contains("CanUseChecked"), "{json}");
        assert!(json.contains("GreeterComponent"), "{json}");
        assert!(json.contains("\"version\":2"), "{json}");
        assert!(json.contains("\"version\":3"), "{json}");
        assert_eq!(json.matches("\"override\":\"Base\"").count(), 2, "{json}");
        assert!(json.contains("\"dependencies\":[\"HasName\"]"), "{json}");
        assert!(!json.contains("Clone"), "{json}");

        let mermaid = render(std::slice::from_ref(&file), GraphFormat::Mermaid);
        assert!(mermaid.starts_with("flowchart LR\n"), "{mermaid}");
        assert!(mermaid.contains("GreeterComponent"), "{mermaid}");
        assert!(mermaid.contains("parent"), "{mermaid}");
        assert!(mermaid.contains("needs"), "{mermaid}");
        assert_eq!(
            mermaid.matches("provides override Base").count(),
            1,
            "{mermaid}"
        );

        let dot = render(std::slice::from_ref(&file), GraphFormat::Dot);
        assert!(dot.starts_with("digraph cgp_wiring {\n"), "{dot}");
        assert!(dot.contains("GreeterComponent"), "{dot}");
        assert!(dot.contains("needs"), "{dot}");
    }
}
