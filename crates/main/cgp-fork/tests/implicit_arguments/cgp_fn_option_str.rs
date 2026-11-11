//! `#[cgp_fn]` with an `Option<&str>` `#[implicit]` argument.
//!
//! `Option<&str>` reads any `Option<U>` whose `U: AsRef<str>`, through `OptionAsRef`.
//! An `Option<String>` field still satisfies that bound. A getter of `Option<&str>`
//! keeps the `Option<String>` field; this test is the implicit form.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_fn.md and
//! cgp-knowledge-base-fork/cgp/reference/attributes/implicit.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    pub fn describe(&self, #[implicit] nickname: Option<&str>) -> String {
        match nickname {
            Some(name) => format!("aka {name}"),
            None => "anonymous".to_owned(),
        }
    }

    expand_describe(output) {
        insta::assert_snapshot!(output, @r#"
        /// The trait for the `describe` operation.
        pub trait Describe {
            /// The `describe` operation.
            fn describe(&self) -> String;
        }
        impl<__Context__> Describe for __Context__
        where
            Self: HasField<
                Symbol<
                    8,
                    Chars<
                        'n',
                        Chars<
                            'i',
                            Chars<
                                'c',
                                Chars<'k', Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                            >,
                        >,
                    >,
                >,
                Value: OptionAsRef<str> + 'static,
            >,
        {
            fn describe(&self) -> String {
                let nickname: Option<&str> = OptionAsRef::option_as_ref(
                    HasField::get_field(
                        self,
                        ::core::marker::PhantomData::<
                            Symbol<
                                8,
                                Chars<
                                    'n',
                                    Chars<
                                        'i',
                                        Chars<
                                            'c',
                                            Chars<
                                                'k',
                                                Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    ),
                );
                match nickname {
                    Some(name) => format!("aka {name}"),
                    None => "anonymous".to_owned(),
                }
            }
        }
        "#)
    }
}

#[derive(HasField)]
pub struct Profile {
    pub nickname: Option<String>,
}

#[test]
fn test_describe() {
    let named = Profile {
        nickname: Some("ada".to_owned()),
    };
    assert_eq!(named.describe(), "aka ada");

    let anon = Profile { nickname: None };
    assert_eq!(anon.describe(), "anonymous");
}
