use proc_macro2::TokenStream;
use syn::{Ident, ItemTrait, parse2};

use crate::ir::{ComponentDef, Diagnostics, FamilyIr, IrFamily, Pipeline, run};
use crate::macro_core::types::cgp_component::{CgpComponentRawArgs, ItemCgpComponent};
use crate::macro_core::types::cgp_type::extract_item_type_from_trait;

/// `#[cgp_type]`: a trait with one associated type becomes an abstract-type component.
pub struct CgpTypeMacro;

impl Pipeline for CgpTypeMacro {
    const NAME: &'static str = "cgp_type";
    const FAMILY: IrFamily = ComponentDef::FAMILY;
    type Ir = ComponentDef;

    /// Parses the component, naming the provider `{Type}TypeProvider` by default.
    fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self::Ir> {
        parse_abstract_type(attr, body).map(ComponentDef::AbstractType)
    }

    fn validate(ir: &Self::Ir, diagnostics: &mut Diagnostics) {
        ir.validate(diagnostics);
    }

    fn emit(ir: Self::Ir) -> syn::Result<TokenStream> {
        ir.emit()
    }
}

/// Parses an abstract-type component, naming the provider `{Type}TypeProvider` by default.
/// Shared with `#[cgp_getter]` on a trait that only declares an associated type.
pub(crate) fn parse_abstract_type(
    attr: TokenStream,
    body: TokenStream,
) -> syn::Result<ItemCgpComponent> {
    let mut raw_args: CgpComponentRawArgs = parse2(attr)?;

    let item_trait: ItemTrait = parse2(body)?;

    let item_type = extract_item_type_from_trait(&item_trait)?;

    if raw_args.provider_ident.is_none() {
        raw_args.provider_ident = Some(Ident::new(
            &format!("{}TypeProvider", item_type.ident),
            item_type.ident.span(),
        ));
    }

    Ok(ItemCgpComponent {
        args: raw_args.try_into()?,
        item_trait,
    })
}

/// `cgp_type` entry point.
pub fn cgp_type(attrs: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    run::<CgpTypeMacro>(attrs, body)
}
