use proc_macro2::Span;
use syn::{Attribute, Type};

use crate::macro_core::types::check_components::TypeWithGenerics;

pub struct EvaluatedCheckEntry {
    pub key: Type,
    pub value: Option<TypeWithGenerics>,
    pub span: Span,
    pub cfgs: Vec<Attribute>,
}
