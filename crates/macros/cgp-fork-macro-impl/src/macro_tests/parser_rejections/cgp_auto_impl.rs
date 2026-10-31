//! `#[cgp_auto_impl]` rejects an attribute argument, a method with no body, and
//! a trait item it cannot turn into a blanket provider.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_impl.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_impl with an attribute argument", || {
        crate::macro_lib::cgp_auto_impl(
            quote!(Context),
            quote!(
                pub trait CanGreet {
                    fn greet(&self) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_method_without_body() {
    // The method body is the blanket provider, so a signature with no body has
    // nothing to move onto the impl.
    assert_macro_rejects("cgp_auto_impl method without a body", || {
        crate::macro_lib::cgp_auto_impl(
            quote!(),
            quote!(
                pub trait CanGreet {
                    fn greet(&self);
                }
            ),
        )
    });
}

#[test]
fn rejects_const_without_value() {
    assert_macro_rejects("cgp_auto_impl const without a value", || {
        crate::macro_lib::cgp_auto_impl(
            quote!(),
            quote!(
                pub trait HasLimit {
                    const LIMIT: usize;
                }
            ),
        )
    });
}

#[test]
fn rejects_macro_item_with_its_message() {
    super::assert_macro_rejects_with(
        "cgp_auto_impl with a macro item",
        "unsupported trait item in blanket impl",
        || {
            crate::macro_lib::cgp_auto_impl(
                quote!(),
                quote!(
                    pub trait CanGreet {
                        fn greet(&self) {}
                        some_macro!();
                    }
                ),
            )
        },
    );
}

/// Every item kind the blanket provider accepts, in one trait: a method body, a
/// const value, a bounded associated type, and a supertrait. The bodies move onto
/// the impl, the trait keeps only signatures, the associated type becomes an impl
/// generic with its bound in the `where` clause, and the supertrait is lowered
/// onto the context.
#[test]
fn moves_each_provided_item_onto_the_blanket_impl() {
    let tokens = crate::macro_lib::cgp_auto_impl(
        quote!(),
        quote!(
            pub trait CanDescribe: HasName {
                type Label: Clone;
                const LIMIT: usize = 3;
                fn describe(&self) -> usize {
                    Self::LIMIT
                }
            }
        ),
    )
    .expect("a trait with provided items expands")
    .to_string();

    assert!(tokens.contains("const LIMIT : usize ;"), "{tokens}");
    assert!(
        tokens.contains("fn describe (& self) -> usize ;"),
        "{tokens}"
    );
    assert!(tokens.contains("const LIMIT : usize = 3 ;"), "{tokens}");
    assert!(tokens.contains("type Label = Label ;"), "{tokens}");
    assert!(tokens.contains("Label : Clone"), "{tokens}");
    assert!(tokens.contains("HasName"), "{tokens}");
}
