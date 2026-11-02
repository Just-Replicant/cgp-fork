//! `#[cgp_auto_error]` on an enum: `Display`, `core::error::Error`, and `From`.
//!
//! The impl form of the attribute builds error providers. This form leaves the enum in place
//! and derives the formatting and the source chain (rewrite plan §7.6).

use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{
    Attribute, Error, Fields, Generics, Ident, ItemEnum, LitStr, Meta, Type, Variant,
    WherePredicate, parse_quote,
};

use crate::ir::Diagnostics;

/// One `#[cgp_auto_error]` enum, ready to validate and emit.
pub struct ItemCgpErrorEnum {
    item: ItemEnum,
    variants: Vec<VariantIr>,
    issues: Vec<Issue>,
}

struct Issue {
    span: Span,
    message: String,
}

enum DisplayKind {
    Missing,
    Invalid,
    Format(LitStr),
    Transparent,
}

struct VariantIr {
    display: DisplayKind,
    /// Index of the `#[source]` / `#[from]` field. A transparent variant uses its only field.
    source: Option<usize>,
    from: Option<usize>,
}

struct Slot {
    /// Field name, or the tuple index as decimal text.
    name: String,
    debug: bool,
}

impl ItemCgpErrorEnum {
    /// Reads the enum. Attribute mistakes are kept for [`Self::validate`] so every one is reported.
    pub fn from_enum(item: ItemEnum) -> Self {
        let mut issues = Vec::new();
        let variants = item
            .variants
            .iter()
            .map(|variant| read_variant(variant, &mut issues))
            .collect();
        Self {
            item,
            variants,
            issues,
        }
    }

    /// Reports every problem with the enum: missing messages, source fields, and placeholders.
    pub fn validate(&self, diagnostics: &mut Diagnostics) {
        for issue in &self.issues {
            diagnostics.push(Error::new(issue.span, &issue.message));
        }

        if self.item.variants.is_empty() {
            diagnostics.push(Error::new_spanned(
                &self.item.ident,
                "an error enum needs at least one variant",
            ));
        }
        let mut from_types: BTreeMap<String, Span> = BTreeMap::new();

        for (variant, ir) in self.item.variants.iter().zip(&self.variants) {
            let fields = field_count(&variant.fields);
            match &ir.display {
                DisplayKind::Missing => diagnostics.push(Error::new_spanned(
                    &variant.ident,
                    "every variant needs #[error(\"...\")] or #[error(transparent)]",
                )),
                DisplayKind::Invalid => {}
                DisplayKind::Transparent => {
                    if fields != 1 {
                        diagnostics.push(Error::new_spanned(
                            &variant.ident,
                            "#[error(transparent)] needs a variant with exactly one field",
                        ));
                    }
                }
                DisplayKind::Format(lit) => {
                    if let Err(error) = check_format(lit, variant) {
                        diagnostics.push(error);
                    }
                }
            }

            if ir.from.is_some() && fields != 1 {
                diagnostics.push(Error::new_spanned(
                    &variant.ident,
                    "#[from] is only valid on a variant with exactly one field",
                ));
            }

            if let Some(index) = ir.source
                && let Some(ty) = field_ty(&variant.fields, index)
                && matches!(ty, Type::Reference(_))
            {
                diagnostics.push(Error::new_spanned(
                    ty,
                    "a source field must be 'static so core::error::Error::source can return it",
                ));
            }

            if let Some(index) = ir.from
                && let Some(ty) = field_ty(&variant.fields, index)
            {
                let key = quote!(#ty).to_string();
                if from_types.insert(key, ty.span()).is_some() {
                    diagnostics.push(Error::new_spanned(
                        ty,
                        "another variant already implements From for this type",
                    ));
                }
            }
        }
    }

    /// The enum, plus `Display`, `Error`, and one `From` per `#[from]` field.
    pub fn emit(self) -> syn::Result<TokenStream> {
        let mut item = self.item.clone();
        strip_enum(&mut item);

        let name = &item.ident;
        let display_generics = bounds_for(&item.generics, &self, false);
        let error_generics = bounds_for(&item.generics, &self, true);
        let (display_impl, _, display_where) = display_generics.split_for_impl();
        let (error_impl, _, error_where) = error_generics.split_for_impl();
        let (_, ty_generics, _) = item.generics.split_for_impl();

        let display_arms = self
            .item
            .variants
            .iter()
            .zip(&self.variants)
            .map(|(variant, ir)| display_arm(variant, ir))
            .collect::<syn::Result<Vec<_>>>()?;
        let source_arms = self
            .item
            .variants
            .iter()
            .zip(&self.variants)
            .map(|(variant, ir)| source_arm(variant, ir))
            .collect::<syn::Result<Vec<_>>>()?;
        let from_impls = self
            .item
            .variants
            .iter()
            .zip(&self.variants)
            .filter_map(|(variant, ir)| ir.from.map(|index| from_impl(&item, variant, index)))
            .collect::<syn::Result<Vec<_>>>()?;

        let source_method = if self.variants.iter().any(|ir| ir.source.is_some()) {
            quote! {
                fn source(&self) -> ::core::option::Option<&(dyn ::core::error::Error + 'static)> {
                    match self {
                        #(#source_arms)*
                    }
                }
            }
        } else {
            TokenStream::new()
        };

        Ok(quote! {
            #item

            impl #display_impl ::core::fmt::Display for #name #ty_generics #display_where {
                fn fmt(&self, __cgp_formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #(#display_arms)*
                    }
                }
            }

            impl #error_impl ::core::error::Error for #name #ty_generics #error_where {
                #source_method
            }

            #(#from_impls)*
        })
    }
}

/// `for_error` selects the `Error` bounds. Otherwise the bounds are for `Display`.
fn bounds_for(generics: &Generics, item: &ItemCgpErrorEnum, for_error: bool) -> Generics {
    let params = generic_params(generics);
    let mut needed: BTreeMap<String, (Ident, BTreeSet<&'static str>)> = BTreeMap::new();

    for (variant, ir) in item.item.variants.iter().zip(&item.variants) {
        if !for_error {
            match &ir.display {
                DisplayKind::Format(lit) => {
                    if let Ok((slots, _)) = parse_format(lit) {
                        for slot in slots {
                            if let Some(ty) = field_by_slot(&variant.fields, &slot.name) {
                                let trait_name = if slot.debug { "debug" } else { "display" };
                                remember(&params, ty, trait_name, &mut needed);
                            }
                        }
                    }
                }
                DisplayKind::Transparent => {
                    if let Some(ty) = field_ty(&variant.fields, 0) {
                        remember(&params, ty, "display", &mut needed);
                    }
                }
                DisplayKind::Missing | DisplayKind::Invalid => {}
            }
        } else if let Some(index) = ir.source
            && let Some(ty) = field_ty(&variant.fields, index)
        {
            remember(&params, ty, "error", &mut needed);
        }
    }

    if for_error {
        let used = field_type_params(&item.item);
        for param in item.item.generics.type_params() {
            if used.contains(&param.ident.to_string()) {
                needed
                    .entry(param.ident.to_string())
                    .or_insert_with(|| (param.ident.clone(), BTreeSet::new()))
                    .1
                    .insert("debug");
            }
        }
    }

    let mut generics = generics.clone();
    for (ident, traits) in needed.into_values() {
        let bounds = trait_bounds(&traits);
        let predicate: WherePredicate = parse_quote! { #ident: #bounds };
        generics.make_where_clause().predicates.push(predicate);
    }
    generics
}

fn trait_bounds(traits: &BTreeSet<&'static str>) -> TokenStream {
    let mut bounds = TokenStream::new();
    for (index, name) in traits.iter().enumerate() {
        if index > 0 {
            bounds.extend(quote! { + });
        }
        bounds.extend(match *name {
            "display" => quote! { ::core::fmt::Display },
            "debug" => quote! { ::core::fmt::Debug },
            _ => quote! { ::core::error::Error + 'static },
        });
    }
    bounds
}

fn remember(
    params: &BTreeSet<String>,
    ty: &Type,
    trait_name: &'static str,
    needed: &mut BTreeMap<String, (Ident, BTreeSet<&'static str>)>,
) {
    let Some(ident) = generic_type(ty, params) else {
        return;
    };
    needed
        .entry(ident.to_string())
        .or_insert_with(|| (ident.clone(), BTreeSet::new()))
        .1
        .insert(trait_name);
}

struct ParamFinder<'a> {
    params: &'a BTreeSet<String>,
    found: BTreeSet<String>,
}

impl Visit<'_> for ParamFinder<'_> {
    fn visit_path(&mut self, path: &syn::Path) {
        if path.segments.len() == 1 && path.segments[0].arguments.is_empty() {
            let ident = path.segments[0].ident.to_string();
            if self.params.contains(&ident) {
                self.found.insert(ident);
            }
        }
        syn::visit::visit_path(self, path);
    }
}

fn field_type_params(item: &ItemEnum) -> BTreeSet<String> {
    let params = generic_params(&item.generics);
    let mut finder = ParamFinder {
        params: &params,
        found: BTreeSet::new(),
    };
    for variant in &item.variants {
        for field in &variant.fields {
            finder.visit_type(&field.ty);
        }
    }
    finder.found
}

fn generic_params(generics: &Generics) -> BTreeSet<String> {
    generics
        .type_params()
        .map(|param| param.ident.to_string())
        .collect()
}

fn generic_type(ty: &Type, params: &BTreeSet<String>) -> Option<Ident> {
    let Type::Path(path) = ty else {
        return None;
    };
    if path.qself.is_some() || path.path.segments.len() != 1 {
        return None;
    }
    let segment = &path.path.segments[0];
    if !segment.arguments.is_empty() || !params.contains(&segment.ident.to_string()) {
        return None;
    }
    Some(segment.ident.clone())
}

fn display_arm(variant: &Variant, ir: &VariantIr) -> syn::Result<TokenStream> {
    let cfgs = cfg_attrs(&variant.attrs);
    let arm = match &ir.display {
        DisplayKind::Transparent => {
            let pattern = bind_field(variant, 0, "inner")?;
            quote! { #pattern => ::core::fmt::Display::fmt(inner, __cgp_formatter), }
        }
        DisplayKind::Format(lit) => {
            let (slots, rewritten) = parse_format(lit)?;
            let used = slots
                .iter()
                .map(|slot| slot.name.clone())
                .collect::<Vec<_>>();
            let pattern = bind_slots(variant, &used)?;
            let format = LitStr::new(&rewritten, lit.span());
            if slots.is_empty() {
                quote! { #pattern => __cgp_formatter.write_str(#format), }
            } else {
                quote! { #pattern => ::core::write!(__cgp_formatter, #format), }
            }
        }
        DisplayKind::Missing | DisplayKind::Invalid => {
            return Err(Error::new_spanned(
                &variant.ident,
                "internal error: emitting an error variant that did not validate",
            ));
        }
    };
    Ok(quote! { #(#cfgs)* #arm })
}

fn source_arm(variant: &Variant, ir: &VariantIr) -> syn::Result<TokenStream> {
    let cfgs = cfg_attrs(&variant.attrs);
    let arm = if let Some(index) = ir.source {
        let pattern = bind_field(variant, index, "source")?;
        quote! { #pattern => ::core::option::Option::Some(source), }
    } else {
        let pattern = bind_slots(variant, &[])?;
        quote! { #pattern => ::core::option::Option::None, }
    };
    Ok(quote! { #(#cfgs)* #arm })
}

fn from_impl(item: &ItemEnum, variant: &Variant, index: usize) -> syn::Result<TokenStream> {
    let Some(ty) = field_ty(&variant.fields, index) else {
        return Err(Error::new_spanned(
            &variant.ident,
            "internal error: #[from] field is missing",
        ));
    };
    let name = &item.ident;
    let variant_name = &variant.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let constructor = match &variant.fields {
        Fields::Unnamed(_) => quote! { Self::#variant_name(value) },
        Fields::Named(fields) => {
            let field = fields
                .named
                .get(index)
                .and_then(|field| field.ident.as_ref())
                .ok_or_else(|| {
                    Error::new_spanned(&variant.ident, "internal error: named #[from] field")
                })?;
            quote! { Self::#variant_name { #field: value } }
        }
        Fields::Unit => {
            return Err(Error::new_spanned(
                &variant.ident,
                "#[from] needs a field to convert from",
            ));
        }
    };
    Ok(quote! {
        impl #impl_generics ::core::convert::From<#ty> for #name #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}

fn bind_field(variant: &Variant, index: usize, binding: &str) -> syn::Result<TokenStream> {
    let name = &variant.ident;
    let binding = Ident::new(binding, Span::call_site());
    match &variant.fields {
        Fields::Unnamed(fields) => {
            let binds = (0..fields.unnamed.len()).map(|i| {
                if i == index {
                    quote! { #binding }
                } else {
                    quote! { _ }
                }
            });
            Ok(quote! { Self::#name(#(#binds),*) })
        }
        Fields::Named(fields) => {
            let field = fields
                .named
                .get(index)
                .and_then(|field| field.ident.clone())
                .ok_or_else(|| Error::new_spanned(&variant.ident, "that field is missing"))?;
            if fields.named.len() == 1 {
                Ok(quote! { Self::#name { #field: #binding } })
            } else {
                Ok(quote! { Self::#name { #field: #binding, .. } })
            }
        }
        Fields::Unit => Err(Error::new_spanned(
            &variant.ident,
            "that variant has no fields",
        )),
    }
}

fn bind_slots(variant: &Variant, used: &[String]) -> syn::Result<TokenStream> {
    let name = &variant.ident;
    let used: BTreeSet<&str> = used.iter().map(String::as_str).collect();
    match &variant.fields {
        Fields::Unit => Ok(quote! { Self::#name }),
        Fields::Unnamed(fields) => {
            let binds = (0..fields.unnamed.len()).map(|index| {
                let label = index.to_string();
                if used.contains(label.as_str()) {
                    let ident = format_ident!("__cgp_{index}");
                    quote! { #ident }
                } else {
                    quote! { _ }
                }
            });
            Ok(quote! { Self::#name(#(#binds),*) })
        }
        Fields::Named(fields) => {
            let idents: Vec<&Ident> = fields
                .named
                .iter()
                .filter_map(|field| field.ident.as_ref())
                .collect();
            let all_used = idents
                .iter()
                .all(|ident| used.contains(ident.to_string().as_str()));
            if all_used && !idents.is_empty() {
                Ok(quote! { Self::#name { #(#idents),* } })
            } else {
                let binds = idents
                    .into_iter()
                    .filter(|ident| used.contains(ident.to_string().as_str()));
                Ok(quote! { Self::#name { #(#binds,)* .. } })
            }
        }
    }
}

fn check_format(lit: &LitStr, variant: &Variant) -> syn::Result<()> {
    let (slots, _) = parse_format(lit)?;
    for slot in slots {
        if field_by_slot(&variant.fields, &slot.name).is_none() {
            let spec = if slot.debug { ":?" } else { "" };
            let name = &slot.name;
            let variant = &variant.ident;
            return Err(Error::new_spanned(
                lit,
                format!("`{{{name}{spec}}}` is not a field of `{variant}`"),
            ));
        }
    }
    Ok(())
}

/// Placeholders and the format string `write!` should see. Tuple indexes become `__cgp_N`.
fn parse_format(lit: &LitStr) -> syn::Result<(Vec<Slot>, String)> {
    let text = lit.value();
    let mut slots = Vec::new();
    let mut rewritten = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            if chars.peek() == Some(&'{') {
                chars.next();
                rewritten.push('{');
                continue;
            }
            let mut body = String::new();
            let closed = loop {
                match chars.next() {
                    Some('}') => break true,
                    Some(next) => body.push(next),
                    None => break false,
                }
            };
            if !closed {
                return Err(Error::new_spanned(lit, "unclosed '{' in #[error(...)]"));
            }
            let (name, debug) = if let Some(name) = body.strip_suffix(":?") {
                (name.to_owned(), true)
            } else if body.contains(':') {
                return Err(Error::new_spanned(
                    lit,
                    "#[error(...)] supports {field} and {field:?}, not other format options",
                ));
            } else {
                (body, false)
            };
            if name.is_empty() || name.contains('{') {
                return Err(Error::new_spanned(
                    lit,
                    "empty format placeholder in #[error(...)]",
                ));
            }
            let capture = if name.chars().all(|ch| ch.is_ascii_digit()) {
                format!("__cgp_{name}")
            } else {
                name.clone()
            };
            rewritten.push('{');
            rewritten.push_str(&capture);
            if debug {
                rewritten.push_str(":?");
            }
            rewritten.push('}');
            slots.push(Slot { name, debug });
        } else if ch == '}' {
            if chars.peek() == Some(&'}') {
                chars.next();
                rewritten.push('}');
            } else {
                return Err(Error::new_spanned(lit, "unmatched '}' in #[error(...)]"));
            }
        } else {
            rewritten.push(ch);
        }
    }
    Ok((slots, rewritten))
}

fn read_variant(variant: &Variant, issues: &mut Vec<Issue>) -> VariantIr {
    let (display, display_issues) = read_display(&variant.attrs);
    issues.extend(display_issues);

    let mut source = None;
    let mut from = None;
    for (index, field) in variant.fields.iter().enumerate() {
        for attr in &field.attrs {
            if attr.path().is_ident("source") {
                record_flag(attr, "source", index, &mut source, issues);
            } else if attr.path().is_ident("from") {
                record_flag(attr, "from", index, &mut from, issues);
                if source.is_none() {
                    source = Some(index);
                } else if source != Some(index) {
                    issues.push(Issue {
                        span: attr.span(),
                        message: "a variant has only one source field".to_owned(),
                    });
                }
            }
        }
    }
    if matches!(display, DisplayKind::Transparent)
        && source.is_none()
        && field_count(&variant.fields) == 1
    {
        source = Some(0);
    }
    VariantIr {
        display,
        source,
        from,
    }
}

fn record_flag(
    attr: &Attribute,
    name: &str,
    index: usize,
    slot: &mut Option<usize>,
    issues: &mut Vec<Issue>,
) {
    if !matches!(attr.meta, Meta::Path(_)) {
        issues.push(Issue {
            span: attr.span(),
            message: format!("#[{name}] takes no arguments"),
        });
        return;
    }
    if slot.is_some() {
        issues.push(Issue {
            span: attr.span(),
            message: format!("a variant has only one #[{name}] field"),
        });
        return;
    }
    *slot = Some(index);
}

fn read_display(attrs: &[Attribute]) -> (DisplayKind, Vec<Issue>) {
    let mut kind = DisplayKind::Missing;
    let mut issues = Vec::new();
    let mut seen = false;
    for attr in attrs {
        if !attr.path().is_ident("error") {
            continue;
        }
        if seen {
            issues.push(Issue {
                span: attr.span(),
                message: "a variant has only one #[error(...)]".to_owned(),
            });
            continue;
        }
        seen = true;
        match parse_error_attr(attr) {
            Ok(parsed) => kind = parsed,
            Err(error) => {
                issues.push(Issue {
                    span: error.span(),
                    message: error.to_string(),
                });
                kind = DisplayKind::Invalid;
            }
        }
    }
    (kind, issues)
}

fn parse_error_attr(attr: &Attribute) -> syn::Result<DisplayKind> {
    attr.parse_args_with(|input: ParseStream| {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            if !input.is_empty() {
                return Err(input.error("#[error(...)] takes one string"));
            }
            return Ok(DisplayKind::Format(lit));
        }
        let ident: Ident = input.parse()?;
        if ident == "transparent" && input.is_empty() {
            return Ok(DisplayKind::Transparent);
        }
        Err(Error::new(
            ident.span(),
            "#[error(...)] expects a string or `transparent`",
        ))
    })
}

fn strip_enum(item: &mut ItemEnum) {
    for variant in &mut item.variants {
        variant.attrs.retain(|attr| !attr.path().is_ident("error"));
        for field in &mut variant.fields {
            field
                .attrs
                .retain(|attr| !attr.path().is_ident("source") && !attr.path().is_ident("from"));
        }
    }
}

fn cfg_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr"))
        .collect()
}

fn field_count(fields: &Fields) -> usize {
    match fields {
        Fields::Unit => 0,
        Fields::Unnamed(fields) => fields.unnamed.len(),
        Fields::Named(fields) => fields.named.len(),
    }
}

fn field_ty(fields: &Fields, index: usize) -> Option<&Type> {
    match fields {
        Fields::Unnamed(fields) => fields.unnamed.get(index).map(|field| &field.ty),
        Fields::Named(fields) => fields.named.get(index).map(|field| &field.ty),
        Fields::Unit => None,
    }
}

fn field_by_slot<'a>(fields: &'a Fields, name: &str) -> Option<&'a Type> {
    match fields {
        Fields::Named(named) => named.named.iter().find_map(|field| {
            field
                .ident
                .as_ref()
                .is_some_and(|ident| ident == name)
                .then_some(&field.ty)
        }),
        Fields::Unnamed(unnamed) => name
            .parse::<usize>()
            .ok()
            .and_then(|index| unnamed.unnamed.get(index).map(|field| &field.ty)),
        Fields::Unit => None,
    }
}
