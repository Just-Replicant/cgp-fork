//! Representative `cgp-fork` behavior tests, compiled against the facade
//! imported under the name `cgp`. See `src/lib.rs`.
#![allow(dead_code, clippy::disallowed_names)]

mod renamed {
    pub mod abstract_types_use_type_component;
    pub mod async_and_send_async_trait_default_body;
    pub mod basic_delegation_shorthand_wiring;
    pub mod dispatching_auto_dispatch_shape;
    pub mod error_backends_std_raise_and_wrap;
    pub mod extensible_records_record_build_with_handlers;
    pub mod extensible_variants_shape_dispatch;
    pub mod field_access_symbol;
    pub mod getters_assoc_type_only;
    pub mod handlers_computer_macro;
    pub mod handlers_pipe_handlers;
    pub mod higher_order_providers_derive_with_provider;
    pub mod implicit_arguments_cgp_fn_field;
    pub mod monadic_handlers_ok_monadic;
    pub mod namespaces_default_impls2;
    pub mod namespaces_path_macro;
    pub mod presets_combine;
    pub mod runtime_has_runtime_mut;
    pub mod shape_macros_struct_round_trip;
}
