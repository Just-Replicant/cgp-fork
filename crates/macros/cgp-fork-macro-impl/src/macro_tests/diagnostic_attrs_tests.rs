//! The `#[diagnostic::on_unimplemented]` messages the macros attach to generated traits
//! (rewrite plan phase 2).

use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Item, ItemTrait};

use crate::macro_core::functions::is_on_unimplemented;

fn traits(output: TokenStream) -> Vec<ItemTrait> {
    let file: File = syn::parse2(output).expect("expansion parses as items");
    file.items
        .into_iter()
        .filter_map(|item| match item {
            Item::Trait(item_trait) => Some(item_trait),
            _ => None,
        })
        .collect()
}

fn messages(item_trait: &ItemTrait) -> Vec<String> {
    item_trait
        .attrs
        .iter()
        .filter(|attr| is_on_unimplemented(attr))
        .map(|attr| quote!(#attr).to_string())
        .collect()
}

fn items_with_messages(output: TokenStream) -> usize {
    let file: File = syn::parse2(output).expect("expansion parses as items");
    file.items
        .iter()
        .filter(|item| {
            let attrs = match item {
                Item::Trait(item) => &item.attrs,
                Item::Impl(item) => &item.attrs,
                Item::Struct(item) => &item.attrs,
                _ => return false,
            };
            attrs.iter().any(is_on_unimplemented)
        })
        .count()
}

#[test]
fn component_traits_name_the_component() {
    let output = crate::macro_lib::cgp_component(
        quote!(Greeter),
        quote!(
            pub trait CanGreet {
                fn greet(&self) -> String;
            }
        ),
    )
    .expect("expands");

    let traits = traits(output.clone());
    let [consumer, provider] = traits.as_slice() else {
        panic!("expected two traits, got {}", traits.len());
    };

    let consumer = messages(consumer).join("\n");
    assert!(
        consumer.contains("does not implement `CanGreet`"),
        "{consumer}"
    );
    assert!(consumer.contains("`GreeterComponent`"), "{consumer}");

    let provider = messages(provider).join("\n");
    assert!(
        provider.contains("does not provide `GreeterComponent` for `{__Context__}`"),
        "{provider}"
    );

    // Only the two traits carry a message; no impl copies it.
    assert_eq!(items_with_messages(output), 2);
}

#[test]
fn a_custom_context_name_is_used_in_the_provider_message() {
    let output = crate::macro_lib::cgp_component(
        quote!(context: Ctx, provider: Greeter),
        quote!(
            pub trait CanGreet {
                fn greet(&self) -> String;
            }
        ),
    )
    .expect("expands");

    let provider = messages(&traits(output)[1]).join("\n");
    assert!(provider.contains("for `{Ctx}`"), "{provider}");
}

#[test]
fn a_user_message_is_kept_on_the_consumer_and_not_copied_to_the_provider() {
    let output = crate::macro_lib::cgp_component(
        quote!(Greeter),
        quote! {
            #[diagnostic::on_unimplemented(message = "custom")]
            pub trait CanGreet { fn greet(&self) -> String; }
        },
    )
    .expect("expands");

    let traits = traits(output);
    let consumer = messages(&traits[0]);
    assert_eq!(consumer.len(), 1, "{consumer:?}");
    assert!(consumer[0].contains("\"custom\""), "{consumer:?}");

    let provider = messages(&traits[1]);
    assert_eq!(provider.len(), 1, "{provider:?}");
    assert!(!provider[0].contains("custom"), "{provider:?}");
}

#[test]
fn auto_getter_names_its_fields() {
    let output = crate::macro_lib::cgp_auto_getter(
        quote!(),
        quote!(
            pub trait HasPerson {
                fn name(&self) -> &str;
                fn age(&self) -> &u8;
            }
        ),
    )
    .expect("expands");

    let message = messages(&traits(output)[0]).join("\n");
    assert!(
        message.contains("does not implement `HasPerson`"),
        "{message}"
    );
    assert!(message.contains("the fields `name`, `age`"), "{message}");
}

#[test]
fn raw_identifiers_are_written_without_the_prefix() {
    let output = crate::macro_lib::cgp_auto_getter(
        quote!(),
        quote!(
            pub trait HasType {
                fn r#type(&self) -> &str;
            }
        ),
    )
    .expect("expands");

    let message = messages(&traits(output)[0]).join("\n");
    assert!(message.contains("the field `type`"), "{message}");
    assert!(!message.contains("r#"), "{message}");
}
