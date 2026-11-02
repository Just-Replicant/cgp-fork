use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, bracketed};

use super::super::cfg_wiring::cfg_attributes;
use crate::macro_core::types::delegate_component::{
    EvalDelegateKey, EvaluatedDelegateKey, SingleDelegateKey,
};

/// The array-key form `[A, B]`, evaluating to one key per bracketed element so
/// several components share a single value.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct MultiDelegateKey {
    pub attributes: Vec<Attribute>,
    pub keys: Punctuated<SingleDelegateKey, Comma>,
}

impl Parse for MultiDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let body;
        bracketed!(body in input);
        let keys = Punctuated::parse_terminated(&body)?;

        Ok(Self { attributes, keys })
    }
}

impl EvalDelegateKey for MultiDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        let mut keys = Vec::new();
        let outer = cfg_attributes(&self.attributes)?;

        for key in &self.keys {
            let mut cfgs = outer.clone();
            cfgs.extend(cfg_attributes(&key.attributes)?);

            let key = EvaluatedDelegateKey {
                generics: key.generics.generics.clone(),
                span: key.ty.span(),
                key: key.ty.clone(),
                cfgs,
            };

            keys.push(key);
        }

        Ok(keys)
    }
}
