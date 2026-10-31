use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote_spanned};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    AngleBracketedGenericArguments, Expr, ExprLit, GenericArgument, Ident, Lit, LitChar, LitInt,
    LitStr, Path, PathArguments, Token, Type, TypePath, parse_quote,
};

use crate::macro_core::traits::ToType;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Symbol {
    pub ident: String,
    pub span: Span,
}

impl Symbol {
    pub fn from_ident(ident: Ident) -> Self {
        Self {
            // `unraw` strips a raw identifier's `r#` prefix so a field written
            // as `r#type` is tagged by its logical name `"type"`, matching what
            // `Symbol!("type")` produces.
            ident: ident.unraw().to_string(),
            span: ident.span(),
        }
    }
}

impl Parse for Symbol {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let literal: LitStr = input.parse()?;
        Ok(Self {
            ident: literal.value(),
            span: literal.span(),
        })
    }
}

impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use crate::macro_core::exports::{Chars, Nil, Symbol};

        let span = self.span;
        let str_value = self.ident.as_str();

        let chars = str_value.chars().rev().fold(
            quote_spanned!(span => #Nil),
            |acc, current| quote_spanned!(span => #Chars < #current, #acc >),
        );

        let len = Literal::usize_unsuffixed(str_value.len());

        let out = quote_spanned! { span => #Symbol < #len, #chars > };

        tokens.extend(out);
    }
}

impl ToType for Symbol {
    /// Builds the same type as [`ToTokens`] directly, one `Chars<…>` level per character.
    ///
    /// Re-parsing the tokens (`parse_quote!(#self)`) recursed once per character inside `syn`
    /// and overflowed the stack for names of a few hundred characters (found by `cargo fuzz`,
    /// `cgp_namespace`). The loop below has no recursion; the spans match the token form.
    fn to_type(&self) -> Type {
        use crate::macro_core::exports::{Chars, Nil, Symbol};

        let span = self.span;
        let chars_path: Path = parse_quote!(#Chars);
        let symbol_path: Path = parse_quote!(#Symbol);

        let generic = |path: &Path, first: GenericArgument, second: Type| -> Type {
            let mut args = Punctuated::new();
            args.push_value(first);
            args.push_punct(Token![,](span));
            args.push_value(GenericArgument::Type(second));

            let mut path = path.clone();
            if let Some(segment) = path.segments.last_mut() {
                segment.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args,
                    gt_token: Token![>](span),
                });
            }
            Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path,
            })
        };
        let literal = |lit: Lit| {
            GenericArgument::Const(Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit,
            }))
        };

        let mut chars: Type = parse_quote!(#Nil);
        for current in self.ident.chars().rev() {
            // `quote!` gives a `char` the call-site span, as the token form does.
            let current = Lit::Char(LitChar::new(current, Span::call_site()));
            chars = generic(&chars_path, literal(current), chars);
        }

        // `quote!` gives an interpolated `Literal` the call-site span too.
        let len = LitInt::new(&self.ident.len().to_string(), Span::call_site());
        generic(&symbol_path, literal(Lit::Int(len)), chars)
    }
}
