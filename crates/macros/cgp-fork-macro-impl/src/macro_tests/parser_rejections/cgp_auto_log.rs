//! `#[cgp_auto_log]` rejects an attribute argument, a method body, a missing
//! receiver, and a trait item that is not a log method.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_log.md.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_argument() {
    assert_macro_rejects("cgp_auto_log with an attribute argument", || {
        crate::extra_macro_lib::cgp_auto_log(
            quote!(Info),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&self, name: &str);
                }
            ),
        )
    });
}

#[test]
fn rejects_method_body() {
    assert_macro_rejects("cgp_auto_log method with a body", || {
        crate::extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&self, name: &str) {}
                }
            ),
        )
    });
}

#[test]
fn rejects_missing_receiver() {
    assert_macro_rejects("cgp_auto_log method without &self", || {
        crate::extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    fn log_hello(name: &str);
                }
            ),
        )
    });
}

#[test]
fn rejects_associated_type() {
    assert_macro_rejects("cgp_auto_log associated type", || {
        crate::extra_macro_lib::cgp_auto_log(
            quote!(),
            quote!(
                pub trait CanLogHello {
                    type Name;
                    fn log_hello(&self, name: &str);
                }
            ),
        )
    });
}

/// One rejection per rule of a logger trait, each pinned to its message.
#[test]
fn rejects_each_malformed_logger_with_its_message() {
    use proc_macro2::TokenStream;

    use super::assert_macro_rejects_with;

    let run = |item: TokenStream| move || crate::extra_macro_lib::cgp_auto_log(quote!(), item);

    let cases: Vec<(&str, &str, TokenStream)> = vec![
        (
            "generic trait",
            "#[cgp_auto_log] does not support generic traits",
            quote!(
                pub trait CanLogHello<T> {
                    fn log_hello(&self, value: T);
                }
            ),
        ),
        (
            "empty trait",
            "a logger trait must contain at least one method",
            quote!(
                pub trait CanLogNothing {}
            ),
        ),
        (
            "colliding detail struct names",
            "this method's detail struct name collides with another log method",
            quote!(
                pub trait CanLog2 {
                    fn log_hello(&self, name: &str);
                    fn log__hello(&self, name: &str);
                }
            ),
        ),
        (
            "async method",
            "log methods must be synchronous safe functions",
            quote!(
                pub trait CanLogHello {
                    async fn log_hello(&self, name: &str);
                }
            ),
        ),
        (
            "unsafe method",
            "log methods must be synchronous safe functions",
            quote!(
                pub trait CanLogHello {
                    unsafe fn log_hello(&self, name: &str);
                }
            ),
        ),
        (
            "generic method",
            "#[cgp_auto_log] does not support generic methods",
            quote!(
                pub trait CanLogHello {
                    fn log_hello<T>(&self, value: T);
                }
            ),
        ),
        (
            "mutable receiver",
            "a log method takes `&self` so it can call `self.log`",
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&mut self, name: &str);
                }
            ),
        ),
        (
            "receiver by value",
            "a log method takes `&self` so it can call `self.log`",
            quote!(
                pub trait CanLogHello {
                    fn log_hello(self, name: &str);
                }
            ),
        ),
        (
            "first argument is not a receiver",
            "a log method takes `&self` so it can call `self.log`",
            quote!(
                pub trait CanLogHello {
                    fn log_hello(name: &str, other: u8);
                }
            ),
        ),
        (
            "destructured argument",
            "log arguments must be plain identifiers",
            quote!(
                pub trait CanLogPair {
                    fn log_pair(&self, (a, b): (u8, u8));
                }
            ),
        ),
        (
            "by-reference binding",
            "log arguments must be plain identifiers",
            quote!(
                pub trait CanLogHello {
                    fn log_hello(&self, ref name: String);
                }
            ),
        ),
    ];

    for (label, message, item) in cases {
        assert_macro_rejects_with(label, message, run(item));
    }
}
