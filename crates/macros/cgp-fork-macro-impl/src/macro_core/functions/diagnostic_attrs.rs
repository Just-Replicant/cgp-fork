//! `#[diagnostic::on_unimplemented]` attributes for the traits the macros generate, so a
//! missing implementation names the component and what to wire instead of a type-level
//! string (rewrite plan phase 2, `docs/rewrite-plan.md` §7.2).

use syn::ext::IdentExt;
use syn::{Attribute, Ident, ItemTrait, LitStr, parse_quote};

/// Whether `attr` is a `#[diagnostic::on_unimplemented]`.
pub fn is_on_unimplemented(attr: &Attribute) -> bool {
    let segments: Vec<String> = attr
        .path()
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    segments == ["diagnostic", "on_unimplemented"]
}

/// Whether `attrs` already holds a `#[diagnostic::on_unimplemented]`.
pub fn has_on_unimplemented(attrs: &[Attribute]) -> bool {
    attrs.iter().any(is_on_unimplemented)
}

/// Adds `attr` to `item_trait` unless the trait already has an `on_unimplemented`
/// message, which the user's own message then keeps.
pub fn add_on_unimplemented(item_trait: &mut ItemTrait, attr: Attribute) {
    if !has_on_unimplemented(&item_trait.attrs) {
        item_trait.attrs.push(attr);
    }
}

/// The message on a component's consumer trait (`CanGreet`): the trait is implemented
/// through the component's wiring.
pub fn consumer_on_unimplemented(consumer: &Ident, component: &Ident) -> Attribute {
    let consumer = consumer.unraw();
    let component = component.unraw();
    let message = lit(format!("`{{Self}}` does not implement `{consumer}`"));
    let label = lit(format!("`{consumer}` is not implemented for `{{Self}}`"));
    let wiring = lit(format!(
        "`{consumer}` is implemented through the component `{component}`: wire a provider for \
         `{component}` in the `delegate_components!` table of `{{Self}}`"
    ));
    let check = lit(format!(
        "if a provider is wired, `check_components!` on `{{Self}}` with `{component}` names \
         the requirement it is missing"
    ));
    parse_quote! {
        #[diagnostic::on_unimplemented(
            message = #message,
            label = #label,
            note = #wiring,
            note = #check
        )]
    }
}

/// The message on a component's provider trait (`Greeter<Context>`), whose leading
/// parameter `context` is the context.
///
/// `Self` is a provider, or a context or wiring table that forwards the component
/// through its `DelegateComponent` entry; the notes cover both.
pub fn provider_on_unimplemented(
    provider: &Ident,
    component: &Ident,
    context: &Ident,
) -> Attribute {
    let provider = provider.unraw();
    let component = component.unraw();
    let context = context.unraw();
    let message = lit(format!(
        "`{{Self}}` does not provide `{component}` for `{{{context}}}`"
    ));
    let label = lit(format!(
        "`{{Self}}` neither implements `{provider}` nor wires `{component}`"
    ));
    let wiring = lit(format!(
        "if `{{Self}}` is a context or a wiring table, add `{component}: SomeProvider` to its \
         `delegate_components!` table"
    ));
    let provider_note = lit(format!(
        "if `{{Self}}` is a provider, implement `{provider}` for it with `#[cgp_impl]` or \
         `#[cgp_provider({component})]`"
    ));
    parse_quote! {
        #[diagnostic::on_unimplemented(
            message = #message,
            label = #label,
            note = #wiring,
            note = #provider_note
        )]
    }
}

/// The message on a `#[cgp_auto_getter]` trait, naming the fields it reads.
pub fn auto_getter_on_unimplemented(getter: &Ident, fields: &[&Ident]) -> Attribute {
    let getter = getter.unraw();
    let names: Vec<String> = fields
        .iter()
        .map(|field| format!("`{}`", field.unraw()))
        .collect();
    let reads = match names.as_slice() {
        [one] => format!("the field {one}"),
        _ => format!("the fields {}", names.join(", ")),
    };
    let message = lit(format!("`{{Self}}` does not implement `{getter}`"));
    let label = lit(format!("`{getter}` needs {reads} on `{{Self}}`"));
    let note = lit(format!(
        "`{getter}` reads {reads} from `{{Self}}`: add it with the type the getter returns and \
         derive `HasField` on `{{Self}}` (`#[derive(HasField)]`)"
    ));
    parse_quote! {
        #[diagnostic::on_unimplemented(message = #message, label = #label, note = #note)]
    }
}

fn lit(value: String) -> LitStr {
    LitStr::new(&value, proc_macro2::Span::call_site())
}
