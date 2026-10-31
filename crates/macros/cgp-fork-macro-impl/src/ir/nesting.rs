//! A depth limit on macro input, checked before parsing.
//!
//! `syn` parses nested types and groups recursively. A type nested a few hundred levels deep
//! (`A<A<A<…>>>`) overflows the stack of the compiler thread the macro runs on, and rustc
//! aborts with `SIGSEGV` instead of reporting an error (found by `cargo fuzz`; with the macro
//! built in the default `dev` profile the crash starts at about 400 levels of generics, while
//! rustc on its own accepts 2000). Checking the depth first turns that crash into a normal
//! compile error.
//!
//! The depth counts delimited groups (`( )`, `[ ]`, `{ }`), open angle brackets, and the
//! current run of prefix operators. A `<` opens and a `>` closes; `->`, `=>`, `<=` and `>=` are
//! skipped, and the angle count goes back to its value at the start of the enclosing group when
//! the group ends and at every `;`, so comparisons in a function body do not add up. A run of
//! prefix operators (`&&&&T`, `&'a mut &'a mut T`, `*const *const T`, `!!!x`, `--x`) is
//! parsed recursively too, so each `&`, `*`, `!` or `-` in a row adds a level; `mut`, `const`
//! and lifetimes inside the run do not end it, anything else does. Other recursive syntax
//! is a `syn` limit shared by every `syn`-based macro.

use proc_macro2::{Spacing, Span, TokenStream, TokenTree};
use syn::Error;

/// The deepest nesting a macro input may have. Far above anything written by hand, and well
/// below the depth at which `syn` overflows a compiler thread's stack.
pub const MAX_NESTING_DEPTH: usize = 128;

/// Returns an error when `tokens` nest deeper than [`MAX_NESTING_DEPTH`].
pub fn check_nesting_depth(tokens: &TokenStream) -> syn::Result<()> {
    match deepest(tokens.clone(), 0) {
        Some(span) => Err(Error::new(
            span,
            format!(
                "this macro input nests more than {MAX_NESTING_DEPTH} levels of brackets or \
                 generic arguments; name an inner part with a type alias to flatten it"
            ),
        )),
        None => Ok(()),
    }
}

/// Walks `tokens` starting at `depth` and returns the span of the first token past the limit.
fn deepest(tokens: TokenStream, depth: usize) -> Option<Span> {
    let mut angles = 0usize;
    let mut prefix = 0usize;
    let mut previous: Option<(char, Spacing)> = None;

    for tree in tokens {
        let current = depth + angles + prefix;
        if current > MAX_NESTING_DEPTH {
            return Some(tree.span());
        }

        let mut this = None;
        match &tree {
            TokenTree::Group(group) => {
                if let Some(span) = deepest(group.stream(), current + 1) {
                    return Some(span);
                }
                prefix = 0;
            }
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                let joined_to_previous = matches!(previous, Some((_, Spacing::Joint)));
                match ch {
                    '<' => angles += 1,
                    '>' if joined_to_previous && matches!(previous, Some(('-' | '=', _))) => {}
                    '>' => angles = angles.saturating_sub(1),
                    '=' if joined_to_previous && matches!(previous, Some(('<', _))) => {
                        angles = angles.saturating_sub(1);
                    }
                    '=' if joined_to_previous && matches!(previous, Some(('>', _))) => {
                        angles += 1;
                    }
                    ';' => angles = 0,
                    _ => {}
                }
                match ch {
                    '&' | '*' | '!' | '-' => prefix += 1,
                    '\'' => {}
                    _ => prefix = 0,
                }
                this = Some((ch, punct.spacing()));
            }
            TokenTree::Ident(ident) => {
                let in_prefix_run =
                    ident == "mut" || ident == "const" || matches!(previous, Some(('\'', _)));
                if !in_prefix_run {
                    prefix = 0;
                }
            }
            TokenTree::Literal(_) => prefix = 0,
        }
        previous = this;
    }

    None
}
