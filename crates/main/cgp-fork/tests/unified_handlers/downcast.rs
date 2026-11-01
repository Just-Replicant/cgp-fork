//! `Downcast` and `DispatchHandlers`, the replacements for the deprecated
//! `DowncastAndHandle` and `DispatchMatchers` (ported from
//! `extensible_variants/variant_dispatch.rs`).
//!
//! Like the handlers in a `MatchWith` list, both take the enum's extractor (what
//! `HasExtractor::to_extractor` returns), not the enum itself.

#![deny(deprecated)]

use cgp_fork::handler::Handler;
use cgp_fork::handler::prelude::*;
use cgp_fork::prelude::*;
use futures::executor::block_on;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBarBaz {
    Foo(u64),
    Bar(String),
    Baz(bool),
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub enum FooBar {
    Foo(u64),
    Bar(String),
}

#[cgp_handler]
fn show_foo_bar(input: FooBar) -> String {
    format!("FooBar::{input:?}")
}

#[cgp_handler]
fn show_baz(input: bool) -> String {
    format!("Baz({input:?})")
}

type Handlers = Product![
    OnField<Symbol!("Baz"), FieldValue<ShowBaz>>,
    Downcast<FooBar, ShowFooBar>,
];

pub struct App;

#[test]
fn downcast_runs_the_provider_on_the_subset() {
    assert_eq!(
        Downcast::<FooBar, ShowFooBar>::handle(&App, NoCode, FooBarBaz::Foo(1).to_extractor()).ok(),
        Some("FooBar::Foo(1)".to_owned())
    );
    assert!(
        Downcast::<FooBar, ShowFooBar>::handle(&App, NoCode, FooBarBaz::Baz(true).to_extractor())
            .is_err()
    );
}

#[test]
fn match_with_a_downcast_handler() {
    assert_eq!(
        MatchWith::<Handlers>::handle(&App, NoCode, FooBarBaz::Bar("hello".to_owned())),
        "FooBar::Bar(\"hello\")"
    );
    assert_eq!(
        MatchWith::<Handlers>::handle(&App, NoCode, FooBarBaz::Baz(true)),
        "Baz(true)"
    );
    assert_eq!(
        block_on(MatchWith::<Handlers>::handle_async(
            &App,
            NoCode,
            FooBarBaz::Foo(1)
        )),
        "FooBar::Foo(1)"
    );
}

#[test]
fn dispatch_handlers_keeps_the_remainder() {
    type Partial = DispatchHandlers<Product![Downcast<FooBar, ShowFooBar>]>;

    assert_eq!(
        Partial::handle(&App, NoCode, FooBarBaz::Foo(2).to_extractor()).ok(),
        Some("FooBar::Foo(2)".to_owned())
    );
    assert!(Partial::handle(&App, NoCode, FooBarBaz::Baz(false).to_extractor()).is_err());

    type Complete = DispatchHandlers<Handlers>;
    assert_eq!(
        Complete::handle(&App, NoCode, FooBarBaz::Baz(false).to_extractor()).ok(),
        Some("Baz(false)".to_owned())
    );
    assert_eq!(
        block_on(Complete::handle_async(
            &App,
            NoCode,
            FooBarBaz::Foo(3).to_extractor()
        ))
        .ok(),
        Some("FooBar::Foo(3)".to_owned())
    );
}
