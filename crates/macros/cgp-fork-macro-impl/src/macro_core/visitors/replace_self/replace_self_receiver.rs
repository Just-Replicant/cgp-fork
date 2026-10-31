use proc_macro2::Ident;
use syn::visit_mut::VisitMut;
use syn::{FnArg, Item, Receiver, Signature, Type, parse_quote};

/// Rewrites a method's `self` receiver into an explicit context parameter,
/// preserving the reference and mutability shape (`&self` → `ctx: &Context`,
/// `&mut self` → `ctx: &mut Context`, `self` → `ctx: Context`).
pub struct ReplaceSelfReceiverVisitor<'a> {
    pub replaced_ident: &'a Ident,
    pub replaced_type: &'a Type,
}

impl<'a> VisitMut for ReplaceSelfReceiverVisitor<'a> {
    fn visit_signature_mut(&mut self, sig: &mut Signature) {
        if let Some(arg) = sig.inputs.first_mut()
            && let FnArg::Receiver(receiver) = arg
        {
            *arg = replace_self_receiver(receiver, self.replaced_ident, self.replaced_type);
        }
    }

    fn visit_item_mut(&mut self, _item: &mut Item) {
        // A block-nested item (`impl`, `trait`, `fn`, …) introduces its own
        // `self`, so its receivers belong to that item, not the enclosing
        // context. Stop before descending into it.
    }
}

pub fn replace_self_receiver(
    receiver: &mut Receiver,
    replaced_ident: &Ident,
    replaced_type: &Type,
) -> FnArg {
    match &receiver.kind {
        syn::ReceiverKind::Reference(_, life, mutability) => match (life, mutability) {
            (None, None) => parse_quote!(#replaced_ident : & #replaced_type),
            (Some(life), None) => parse_quote!(#replaced_ident : & #life #replaced_type),
            (None, Some(_mut)) => parse_quote!(#replaced_ident : &mut #replaced_type),
            (Some(life), Some(_mut)) => {
                parse_quote!(#replaced_ident : & #life mut #replaced_type)
            }
        },
        // `self` and `self: Type` both become an owned context parameter. A
        // typed receiver's type is not copied; the caller supplies the context
        // type. `mut` on an owned receiver binds the parameter.
        _ if receiver.mutability.is_some() => {
            parse_quote!(mut #replaced_ident : #replaced_type)
        }
        _ => parse_quote!(#replaced_ident : #replaced_type),
    }
}
