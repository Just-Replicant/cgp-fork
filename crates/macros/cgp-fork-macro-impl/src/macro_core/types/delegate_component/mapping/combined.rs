use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{Attribute, Error, Ident, Path, Type, parenthesized};

use crate::macro_core::traits::PeekKeyword;
use crate::macro_core::types::delegate_component::{
    DelegateKey, DelegateMode, DirectDelegateMapping, EvalDelegateEntries, EvaluatedDelegateEntry,
    ExtractInnerDelegateTables, GetterDelegateShorthand, InnerDelegateTable, NormalDelegateMapping,
    PresetDelegateEntry, RedirectDelegateMapping, TypeDelegateShorthand,
};
use crate::macro_core::types::keyword::Keyword;
use crate::macro_core::types::keywords::{Getter, Override, Preset};

/// One `Key OP Value` entry, its variant chosen by the operator: `:` is Normal,
/// `->` is Direct, `=>` is Redirect.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub enum DelegateMapping {
    Normal(NormalDelegateMapping),
    Direct(DirectDelegateMapping),
    Redirect(RedirectDelegateMapping),
    TypeShorthand(TypeDelegateShorthand),
    GetterShorthand(GetterDelegateShorthand),
    Preset(PresetDelegateEntry),
    /// `override` / `override(Parent)` — a preset entry that must already exist on a parent.
    Override(OverrideDelegateEntry),
}

/// An entry that must name a component a parent preset already wires.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct OverrideDelegateEntry {
    pub span: Span,
    /// `override(Parent)` names the parent. `None` means the preset's only parent.
    pub parent: Option<Path>,
    pub entry: Box<DelegateMapping>,
}

impl Parse for DelegateMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Token![type]) {
            return Ok(Self::TypeShorthand(input.parse()?));
        }

        if peek_bare_keyword::<Getter>(input) {
            return Ok(Self::GetterShorthand(input.parse()?));
        }

        if peek_bare_keyword::<Preset>(input) {
            return Ok(Self::Preset(input.parse()?));
        }

        // `#[cfg_wiring] override Key: Provider` — the attribute is the entry's,
        // and `override` is what the next token is after those attributes.
        let look = input.fork();
        let _ = look.call(Attribute::parse_outer)?;
        if peek_bare_keyword::<Override>(&look) {
            let attributes = input.call(Attribute::parse_outer)?;
            let mut entry: OverrideDelegateEntry = input.parse()?;
            prepend_key_attributes(&mut entry.entry, attributes);
            return Ok(Self::Override(entry));
        }

        let key = input.parse()?;
        let mode: DelegateMode = input.parse()?;

        let entry = match mode {
            DelegateMode::Normal(colon) => {
                let value = input.parse()?;
                Self::Normal(NormalDelegateMapping { key, colon, value })
            }
            DelegateMode::Direct(arrow) => {
                let value = input.parse()?;
                Self::Direct(DirectDelegateMapping { key, arrow, value })
            }
            DelegateMode::Redirect(arrow) => {
                let value = input.parse()?;
                Self::Redirect(RedirectDelegateMapping { key, arrow, value })
            }
        };

        Ok(entry)
    }
}

impl EvalDelegateEntries for DelegateMapping {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        match self {
            Self::Normal(entry) => entry.eval_entries(table_type),
            Self::Direct(entry) => entry.eval_entries(table_type),
            Self::Redirect(entry) => entry.eval_entries(table_type),
            Self::TypeShorthand(entry) => entry.eval_entries(table_type),
            Self::GetterShorthand(entry) => entry.eval_entries(table_type),
            // A preset's keys are not known here; `build_preset_invocations`
            // emits the `with_components!` call that expands them.
            Self::Preset(_) => Ok(Vec::new()),
            Self::Override(entry) => entry.entry.eval_entries(table_type),
        }
    }
}

impl ExtractInnerDelegateTables for DelegateMapping {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        match self {
            Self::Normal(entry) => entry.extract_inner_tables(),
            Self::Direct(entry) => entry.extract_inner_tables(),
            Self::Redirect(_)
            | Self::TypeShorthand(_)
            | Self::GetterShorthand(_)
            | Self::Preset(_) => Vec::new(),
            Self::Override(entry) => entry.entry.extract_inner_tables(),
        }
    }
}

impl Parse for OverrideDelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keyword: Keyword<Override> = input.parse()?;

        let parent = if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            let path = content.parse()?;

            if !content.is_empty() {
                return Err(Error::new(
                    content.span(),
                    "`override(...)` takes one parent preset",
                ));
            }

            Some(path)
        } else {
            None
        };

        let entry: DelegateMapping = input.parse()?;

        if matches!(
            entry,
            DelegateMapping::Override(_) | DelegateMapping::Preset(_)
        ) {
            return Err(Error::new(
                keyword.span,
                "`override` wraps one wiring entry, not another `override` or `preset`",
            ));
        }

        Ok(Self {
            span: keyword.span,
            parent,
            entry: Box::new(entry),
        })
    }
}

fn prepend_key_attributes(mapping: &mut DelegateMapping, attributes: Vec<Attribute>) {
    if attributes.is_empty() {
        return;
    }

    let key = match mapping {
        DelegateMapping::Normal(mapping) => &mut mapping.key,
        DelegateMapping::Direct(mapping) => &mut mapping.key,
        DelegateMapping::Redirect(mapping) => &mut mapping.key,
        DelegateMapping::Override(_)
        | DelegateMapping::TypeShorthand(_)
        | DelegateMapping::GetterShorthand(_)
        | DelegateMapping::Preset(_) => return,
    };

    let slot = match key {
        DelegateKey::Single(key) => &mut key.attributes,
        DelegateKey::Multi(key) => &mut key.attributes,
        DelegateKey::Path(key) => &mut key.attributes,
    };
    let _ = slot.splice(0..0, attributes);
}

/// `getter name` / `preset Path` rather than a key of that name followed by an
/// operator. A following `:`, `->`, or `=>` keeps the explicit `Key OP Value` form.
fn peek_bare_keyword<K>(input: ParseStream) -> bool
where
    K: crate::macro_core::traits::IsKeyword,
{
    if !input.peek_keyword::<K>() {
        return false;
    }

    let fork = input.fork();
    let parsed = if K::IDENT == "override" {
        fork.parse::<syn::Token![override]>().is_ok()
    } else {
        fork.parse::<Ident>().is_ok()
    };
    if !parsed {
        return false;
    }

    !fork.peek(syn::Token![:]) && !fork.peek(syn::Token![->]) && !fork.peek(syn::Token![=>])
}
