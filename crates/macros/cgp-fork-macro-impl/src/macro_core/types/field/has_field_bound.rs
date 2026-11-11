use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;
use syn::token::Mut;

use crate::macro_core::exports::{HasField, HasFieldMut, OptionAsRef};
use crate::macro_core::types::getter::FieldMode;

pub struct HasFieldBound {
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub tag_type: Type,
}

impl ToTokens for HasFieldBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            field_type,
            field_mut,
            field_mode,
            tag_type,
        } = self;

        let output = if field_mut.is_none() {
            if let FieldMode::Slice = field_mode {
                quote! {
                    #HasField< #tag_type, Value: ::core::convert::AsRef< [ #field_type ] > + 'static >
                }
            } else if let FieldMode::OptionStr = field_mode {
                if is_str_type(field_type) {
                    quote! {
                        #HasField< #tag_type, Value: #OptionAsRef< str > + 'static >
                    }
                } else {
                    quote! {
                        #HasField< #tag_type, Value = #field_type >
                    }
                }
            } else if let FieldMode::Str = field_mode {
                if is_str_type(field_type) {
                    // An `#[implicit]` argument borrows any `AsRef<str>` field. The
                    // `'static` bound mirrors the slice case: the generated impl
                    // names the associated type in a `where` clause, which needs it
                    // to outlive the impl.
                    quote! {
                        #HasField< #tag_type, Value: ::core::convert::AsRef< str > + 'static >
                    }
                } else {
                    // A getter keeps the field type itself, because its bound is also
                    // used by the `WithProvider` adapter.
                    quote! {
                        #HasField< #tag_type, Value = #field_type >
                    }
                }
            } else {
                quote! {
                    #HasField< #tag_type, Value = #field_type >
                }
            }
        } else if let FieldMode::Str = field_mode {
            if is_str_type(field_type) {
                quote! {
                    #HasFieldMut< #tag_type, Value: ::core::convert::AsMut< str > + 'static >
                }
            } else {
                quote! {
                    #HasFieldMut< #tag_type, Value = #field_type >
                }
            }
        } else if let FieldMode::Slice = field_mode {
            // A mutable slice reads through `AsMut::as_mut`, so the field's value must
            // implement `AsMut<[T]>` — the mutable mirror of the shared slice's
            // `AsRef<[T]>` bound.
            quote! {
                #HasFieldMut< #tag_type, Value: ::core::convert::AsMut< [ #field_type ] > + 'static >
            }
        } else {
            quote! {
                #HasFieldMut< #tag_type, Value = #field_type >
            }
        };

        tokens.extend(output);
    }
}

/// The borrowed-string field type an `#[implicit]` argument asks for, as opposed
/// to the `String` a getter keeps.
fn is_str_type(field_type: &Type) -> bool {
    match field_type {
        Type::Path(path) => path.qself.is_none() && path.path.is_ident("str"),
        _ => false,
    }
}
