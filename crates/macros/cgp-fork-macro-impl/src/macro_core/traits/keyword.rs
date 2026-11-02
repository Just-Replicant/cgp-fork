use syn::Ident;
use syn::parse::ParseBuffer;

use crate::macro_core::types::keyword::Keyword;

pub trait IsKeyword {
    const IDENT: &'static str;
}

pub trait PeekKeyword {
    fn peek_keyword<K: IsKeyword>(&self) -> bool;
}

impl<'a> PeekKeyword for ParseBuffer<'a> {
    fn peek_keyword<K: IsKeyword>(&self) -> bool {
        // `override` is a reserved keyword, so it is not an `Ident`.
        if K::IDENT == "override" {
            return self.peek(syn::Token![override]);
        }

        if let Ok(ident) = self.fork().parse::<Ident>()
            && ident == K::IDENT
        {
            true
        } else {
            false
        }
    }
}

pub trait ParseOptionalKeyword {
    fn parse_optional_keyword<K: IsKeyword>(&self) -> syn::Result<Option<Keyword<K>>>;
}

impl<'a> ParseOptionalKeyword for ParseBuffer<'a> {
    fn parse_optional_keyword<K: IsKeyword>(&self) -> syn::Result<Option<Keyword<K>>> {
        let keyword = if self.peek_keyword::<K>() {
            Some(self.parse()?)
        } else {
            None
        };

        Ok(keyword)
    }
}
