use syn::parse::{End, Parse, ParseStream};
use syn::token::{Colon, Comma};
use syn::{Error, Ident, LitBool};

use crate::macro_core::types::ident::IdentWithTypeGenerics;

/// The attribute args exactly as written, before defaults are applied. Parses
/// either a bare provider identifier or the `key: value` form; [`CgpComponentArgs`]
/// resolves the defaults via `TryFrom`.
#[derive(Default)]
pub struct CgpComponentRawArgs {
    pub context_ident: Option<Ident>,
    pub provider_ident: Option<Ident>,
    pub component_name: Option<IdentWithTypeGenerics>,
    /// `#[cgp_component(Greeter, mock)]` generates `MockGreeter`.
    pub mock: bool,
}

impl Parse for CgpComponentRawArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(End) {
            let provider_ident = input.parse()?;

            return Ok(CgpComponentRawArgs {
                provider_ident: Some(provider_ident),
                ..Default::default()
            });
        }

        let mut args = Self::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;

            if !input.peek(Colon) {
                if key == "mock" {
                    if args.mock {
                        return Err(Error::new(key.span(), "duplicate `mock`"));
                    }
                    args.mock = true;
                } else if args.provider_ident.is_none() {
                    args.provider_ident = Some(key);
                } else {
                    return Err(Error::new(
                        key.span(),
                        format!("unknown argument `{key}`; write `mock` or `provider: {key}`"),
                    ));
                }
            } else {
                let _: Colon = input.parse()?;

                match key.to_string().as_str() {
                    "name" => {
                        if args.component_name.is_some() {
                            return Err(Error::new(key.span(), "duplicate key is not allowed"));
                        }

                        args.component_name = Some(input.parse()?);
                    }
                    "context" => {
                        if args.context_ident.is_some() {
                            return Err(Error::new(key.span(), "duplicate key is not allowed"));
                        }

                        args.context_ident = Some(input.parse()?);
                    }
                    "provider" => {
                        if args.provider_ident.is_some() {
                            return Err(Error::new(key.span(), "duplicate key is not allowed"));
                        }

                        args.provider_ident = Some(input.parse()?);
                    }
                    "mock" => {
                        if args.mock {
                            return Err(Error::new(key.span(), "duplicate `mock`"));
                        }

                        let value: LitBool = input.parse()?;
                        args.mock = value.value();
                    }
                    _ => {
                        return Err(Error::new(key.span(), format!("unknown key {key}")));
                    }
                }
            }

            if input.parse::<Option<Comma>>()?.is_none() {
                break;
            }
        }

        Ok(args)
    }
}
