# Changelog

## [0.9.1](https://github.com/Just-Replicant/cgp-fork/compare/v0.9.0...v0.9.1) - 2026-10-08

### Changed

- *(delegate-component)* stop clearing the removed const eq token
- *(derive-extractor)* build tuple fields with syn 3 field modifiers
- *(visitors)* ignore the attrs field on syn 3 type paths
- *(elaborate-lifetimes)* visit syn 3 function-pointer types
- *(replace-self)* expand syn 3 receiver kinds into context parameters
- *(implicits)* take receiver mutability from syn 3 reference kinds
- *(getter)* reject unsafe and owned receivers through syn 3
- *(cgp-auto-dispatch)* match syn 3 receiver kinds and fn modifiers
- *(cgp-fn)* add syn 3 trait-fn modifiers and name item variants
- *(cgp-auto-log)* follow syn 3 items, safety, and receivers
- *(cgp-auto-error)* follow syn 3 impl, safety, and type-path shapes
- *(blanket-trait)* emit syn 3 traits, impls, and item modifiers
- *(macro)* name syn 3 item variants instead of Into
- *(provider-impl)* clear defaultness through syn 3 impl modifiers
- *(cgp-provider)* read the syn 3 impl trait pair
- *(cgp-impl)* read the syn 3 impl trait pair
- *(macro)* build item impls with syn 3 headers
- *(cgp-auto-impl)* fill syn 3 impl-item modifiers
- *(delegated-impls)* fill syn 3 impl-item modifiers
- *(cgp-fork-macro)* compile the shared sources from inside the crate
- *(test-util)* move the snapshot helper library under cgp-fork-macro/shared
- *(extra-macro-lib)* move the extra macro entries under cgp-fork-macro/shared
- *(extra-macro-core)* move the extra macro parsers under cgp-fork-macro/shared
- *(macro-lib)* move the macro entries under cgp-fork-macro/shared
- *(macro-core)* move the parser sources under cgp-fork-macro/shared

## v0.9.0 (2026-09-29)

New features:

- In `delegate_components!`, write a type or getter component as the value alone when the provider is `UseType` or `UseField` - [#188](https://github.com/contextgeneric/cgp/issues/188)
- Introduce `cgp_preset!`, so one preset entry expands to many `DelegateComponent` impls and two presets can be combined - [#26](https://github.com/contextgeneric/cgp/issues/26)
- Introduce `#[derive_provider(WithProvider)]`, deriving the `WithProvider` impl instead of writing it by hand - [#30](https://github.com/contextgeneric/cgp/issues/30)
- Introduce `#[cgp_auto_impl]`, turning a trait method body into a blanket provider - [#181](https://github.com/contextgeneric/cgp/issues/181)
- Introduce `#[cgp_auto_error]`, generating `HasErrorType` / `CanRaiseError` / `CanWrapError` providers from one error definition - [#182](https://github.com/contextgeneric/cgp/issues/182)
- Allow a `#[cgp_getter]` / `#[cgp_auto_getter]` trait to consist of an associated type, reusing the `#[cgp_type]` path - [#183](https://github.com/contextgeneric/cgp/issues/183)
- Allow `#[helper]` methods inside `#[cgp_impl]` that are not part of the provider trait - [#184](https://github.com/contextgeneric/cgp/issues/184)
- Introduce `CanLog` and `#[cgp_auto_log]`, a logger component with a blanket impl parallel to `#[cgp_auto_getter]` - [#185](https://github.com/contextgeneric/cgp/issues/185)
- Read `#[implicit]` and `#[field]` arguments from the context in `#[cgp_fn]` and `#[cgp_computer]`, and add `#[derive_promote]` to wrap a `Computer` as a component provider - [#193](https://github.com/contextgeneric/cgp/issues/193)
- Introduce `Struct!` and `Enum!`, writing a struct or enum shape as its declaration instead of a `Product!` or `Sum!` of `Field` entries - [#276](https://github.com/contextgeneric/cgp/pull/276)
- Accept variants with no fields in `#[derive(CgpVariant)]`, `#[derive(CgpData)]`, `#[derive(ExtractField)]`, and `#[derive(FromVariant)]`, with payload `Nil` - [#278](https://github.com/contextgeneric/cgp/pull/278)
