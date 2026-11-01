//! `Symbol::to_type` builds the type-level string without re-parsing it.

use proc_macro2::{Span, TokenStream};
use syn::Type;

use crate::macro_core::traits::ToType;
use crate::macro_core::types::field::Symbol;

fn symbol(name: &str) -> Symbol {
    Symbol {
        ident: name.to_owned(),
        span: Span::call_site(),
    }
}

#[test]
fn to_type_matches_the_token_form() {
    for name in ["", "a", "name", "r#type", "ünïcode", "with space"] {
        let symbol = symbol(name);
        let built = symbol.to_type();
        let parsed: Type = syn::parse_quote!(#symbol);
        assert!(
            crate::macro_core::functions::tokens_eq(&built, &parsed),
            "`{name}`: {} != {}",
            quote::quote!(#built),
            quote::quote!(#parsed)
        );
    }
}

#[test]
fn long_names_do_not_overflow_the_stack() {
    // Found by cargo-fuzz (cgp_namespace): a name of a few hundred characters became a type
    // nested once per character, and re-parsing it overflowed the stack inside `syn`.
    // Run on rustc's 8 MiB compiler-thread stack. Printing and dropping the type still recurse
    // a little per level, which holds 5000 levels; rustc itself crashes at about 3000.
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let ty = symbol(&"x".repeat(5_000)).to_type();
            let tokens: TokenStream = quote::quote!(#ty);
            assert!(tokens.to_string().starts_with(":: cgp_fork"), "{tokens}");
        })
        .unwrap()
        .join()
        .unwrap();
}
