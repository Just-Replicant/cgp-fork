//! Generated public items carry a doc line, and a doc the user wrote on `#[cgp_fn]`
//! stays on the method.

use quote::quote;

use crate::test_util_lib::functions::pretty_format;

#[test]
fn cgp_fn_keeps_the_user_doc_on_the_method() {
    let output = crate::macro_lib::cgp_fn(
        quote!(),
        quote!(
            /// Greets the user.
            pub fn greet(&self) -> String {
                String::new()
            }
        ),
    )
    .unwrap();

    let formatted = pretty_format(output).unwrap();
    assert!(
        formatted.contains("/// Greets the user."),
        "the user's doc should stay on the method:\n{formatted}"
    );
    assert!(
        formatted.contains("The trait for the `greet` operation."),
        "the trait should carry its own doc:\n{formatted}"
    );
    let method_at = formatted.find("fn greet").expect("method");
    let user_doc = formatted.find("Greets the user.").expect("user doc");
    let trait_doc = formatted
        .find("The trait for the `greet` operation.")
        .expect("trait doc");
    assert!(
        trait_doc < method_at && user_doc < method_at && user_doc > trait_doc,
        "trait doc, then the user's doc, then the method:\n{formatted}"
    );
}

#[test]
fn cgp_fn_documents_a_method_that_has_no_doc() {
    let output = crate::macro_lib::cgp_fn(
        quote!(),
        quote!(
            pub fn greet(&self) -> String {
                String::new()
            }
        ),
    )
    .unwrap();
    let formatted = pretty_format(output).unwrap();
    assert!(formatted.contains("The `greet` operation."));
    assert!(formatted.contains("The trait for the `greet` operation."));
}
