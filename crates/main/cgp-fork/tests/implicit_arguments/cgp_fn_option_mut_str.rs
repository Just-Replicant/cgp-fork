//! A mutable `Option<&mut str>` implicit keeps the `Option<String>` field and reads
//! it with `as_deref_mut`. The shared `Option<&str>` form is the one that uses
//! `OptionAsRef`.

use cgp_fork::prelude::*;

#[cgp_component(Namer)]
pub trait CanRename {
    fn rename(&mut self);
}

#[cgp_impl(new RaiseNick)]
impl Namer {
    fn rename(&mut self, #[implicit] nick: Option<&mut str>) {
        if let Some(nick) = nick {
            nick.make_ascii_uppercase();
        }
    }
}

#[derive(HasField)]
pub struct Profile {
    pub nick: Option<String>,
}

delegate_components! {
    Profile {
        NamerComponent: RaiseNick,
    }
}

#[test]
fn mutates_some_and_leaves_none() {
    let mut named = Profile {
        nick: Some("ada".to_owned()),
    };
    named.rename();
    assert_eq!(named.nick.as_deref(), Some("ADA"));

    let mut anon = Profile { nick: None };
    anon.rename();
    assert_eq!(anon.nick, None);
}
