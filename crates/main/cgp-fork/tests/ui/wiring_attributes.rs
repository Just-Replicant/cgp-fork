// Every illegal attribute in a wiring table is reported in one compile, not one per fix.
use cgp_fork::prelude::*;

pub struct App;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

#[cgp_type]
pub trait HasAgeType {
    type Age;
}

delegate_components! {
    App {
        #[first]
        #[second]
        NameTypeProviderComponent: UseType<String>,
        #[third]
        AgeTypeProviderComponent: UseType<u8>,
    }
}

cgp_preset! {
    AppPreset {
        #[in_preset]
        NameTypeProviderComponent: UseType<String>,
        #[also_in_preset]
        AgeTypeProviderComponent: UseType<u8>,
    }
}

pub struct Checked;

// `#[check_trait]` on the table and `#[skip_check]` on a key are legal here.
delegate_and_check_components! {
    #[check_trait(CanUseChecked)]
    Checked {
        #[skip_check]
        #[extra]
        NameTypeProviderComponent: UseType<String>,
        #[unknown]
        AgeTypeProviderComponent: UseType<u8>,
    }
}

fn main() {}
