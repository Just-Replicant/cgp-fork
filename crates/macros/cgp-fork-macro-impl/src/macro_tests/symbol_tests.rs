//! `Symbol::to_type` builds the type-level string without re-parsing it.

use proc_macro2::{Span, TokenStream};
use syn::{GenericArgument, PathArguments, Type};

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

/// Counts `Chars<…>` layers by walking the type in a loop.
///
/// `Drop` and `quote` recurse once per layer. A debug build on Windows overflows an 8 MiB
/// thread around 5000 layers, so the long-name check never quotes or drops that type.
fn chars_layers(ty: &Type) -> usize {
    let mut layers = 0;
    let mut current = ty;
    while let Type::Path(type_path) = current {
        let Some(segment) = type_path.path.segments.last() else {
            break;
        };
        if segment.ident == "Nil" {
            break;
        }
        let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            break;
        };
        let Some(GenericArgument::Type(inner)) = arguments.args.iter().nth(1) else {
            break;
        };
        if segment.ident == "Chars" {
            layers += 1;
        }
        current = inner;
    }
    layers
}

#[test]
fn long_names_do_not_overflow_the_stack() {
    // Found by cargo-fuzz (`cgp_namespace`): a name of a few hundred characters became a type
    // nested once per character, and re-parsing it overflowed the stack inside `syn`.
    // `ToType` builds that type in a loop. 5000 is far past the old crash and past
    // `MAX_IDENT_LEN` (255), which is the longest name a macro will re-parse.
    let ty = symbol(&"x".repeat(5_000)).to_type();
    let Type::Path(outer) = &ty else {
        panic!("symbol type is not a path");
    };
    assert_eq!(
        outer
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .as_deref(),
        Some("Symbol")
    );
    assert_eq!(chars_layers(&ty), 5_000);
    // `Drop` for this type recurses once per character. `ManuallyDrop` skips that walk;
    // the allocation lives until the test process exits.
    let _keep = std::mem::ManuallyDrop::new(ty);

    let ty = symbol(&"x".repeat(crate::ir::MAX_IDENT_LEN)).to_type();
    let tokens: TokenStream = quote::quote!(#ty);
    assert!(tokens.to_string().starts_with(":: cgp_fork"), "{tokens}");
}
