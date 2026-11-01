use syn::token::Mut;
use syn::{Expr, Ident, Stmt, Type};

use crate::macro_core::functions::tokens_eq;
use crate::macro_core::parse_internal;
use crate::macro_core::types::field::{FieldName, HasFieldBound};
use crate::macro_core::types::getter::{FieldMode, GetFieldExpr, GetFieldWithModeExpr};

#[derive(Clone)]
pub struct ImplicitArgField {
    pub field_name: Ident,
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub arg_type: Type,
}

/// Two implicit arguments are the same when they read the same field the same way.
/// Written by hand because `syn::Type: PartialEq` needs syn's `extra-traits`; as
/// with `syn`'s derive, spans are ignored and every `mut` token is equal.
impl PartialEq for ImplicitArgField {
    fn eq(&self, other: &Self) -> bool {
        self.field_name == other.field_name
            && tokens_eq(&self.field_type, &other.field_type)
            && self.field_mut.is_some() == other.field_mut.is_some()
            && self.field_mode == other.field_mode
            && tokens_eq(&self.arg_type, &other.arg_type)
    }
}

impl Eq for ImplicitArgField {}

impl ImplicitArgField {
    pub fn to_has_field_bound(&self) -> syn::Result<HasFieldBound> {
        let field_name = FieldName::from(self.field_name.clone());
        let tag_type = parse_internal!(#field_name);

        Ok(HasFieldBound {
            field_type: self.field_type.clone(),
            field_mut: self.field_mut,
            field_mode: self.field_mode.clone(),
            tag_type,
        })
    }

    /// The field read, converted the way the argument's type requires.
    pub fn to_expr(&self, receiver: &Expr) -> syn::Result<Expr> {
        let get_field_expr = GetFieldWithModeExpr {
            field_mode: self.field_mode.clone(),
            get_field: GetFieldExpr {
                receiver: receiver.clone(),
                field_mut: self.field_mut,
                field_name: self.field_name.clone().into(),
            },
        };

        let expr = parse_internal!( #get_field_expr );
        Ok(expr)
    }

    pub fn to_statement(&self) -> syn::Result<Stmt> {
        let field_name = &self.field_name;
        let arg_type = &self.arg_type;
        let receiver: Expr = parse_internal!(self);
        let get_field_expr = self.to_expr(&receiver)?;

        let statement = parse_internal! {
            let #field_name: #arg_type = #get_field_expr;
        };

        Ok(statement)
    }
}
