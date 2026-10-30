use proc_macro2::{Group, TokenStream, TokenTree};
use quote::quote;

use crate::crate_path::crate_root;

/// Strips the `::cgp_fork::macro_prelude::` prefix (or the renamed crate's equivalent, see
/// [`crate::crate_path`]) from the [`TokenStream`] so the error message shows the more
/// readable, unqualified paths. The replacement is done at the token level, recursing into
/// nested groups.
pub fn strip_macro_prelude(body: TokenStream) -> TokenStream {
    let root = crate_root();
    let prefix: Vec<String> = quote! { #root::macro_prelude:: }
        .into_iter()
        .map(|token| token.to_string())
        .collect();
    strip_prefix(body, &prefix)
}

fn strip_prefix(body: TokenStream, prefix: &[String]) -> TokenStream {
    let is_prefix = |tokens: &[TokenTree]| {
        tokens.len() == prefix.len()
            && tokens.iter().zip(prefix).all(|(token, expected)| {
                !matches!(token, TokenTree::Group(_)) && token.to_string() == *expected
            })
    };

    let tokens: Vec<TokenTree> = body.into_iter().collect();
    let mut output = Vec::with_capacity(tokens.len());
    let mut i = 0;

    while i < tokens.len() {
        if is_prefix(&tokens[i..(i + prefix.len()).min(tokens.len())]) {
            i += prefix.len();
        } else {
            match &tokens[i] {
                TokenTree::Group(group) => {
                    let inner = strip_prefix(group.stream(), prefix);
                    let mut new_group = Group::new(group.delimiter(), inner);
                    new_group.set_span(group.span());
                    output.push(TokenTree::Group(new_group));
                }
                other => output.push(other.clone()),
            }
            i += 1;
        }
    }

    output.into_iter().collect()
}
