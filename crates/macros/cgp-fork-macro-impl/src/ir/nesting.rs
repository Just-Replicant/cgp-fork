//! A depth limit and an identifier-length limit on macro input, checked before parsing.
//!
//! `syn` parses nested types and groups recursively. A type nested a few hundred levels deep
//! (`A<A<A<…>>>`) overflows the stack of the compiler thread the macro runs on, and rustc
//! aborts with `SIGSEGV` instead of reporting an error (found by `cargo fuzz`; with the macro
//! built in the default `dev` profile the crash starts at about 400 levels of generics, while
//! rustc on its own accepts 2000). Checking the depth first turns that crash into a normal
//! compile error.
//!
//! The depth counts delimited groups (`( )`, `[ ]`, `{ }`), open angle brackets, and the
//! current run of prefix operators. A `<` opens and a `>` closes; `->`, `=>` and `<=` are
//! skipped, and open angles are dropped when the enclosing group ends and at every `;`, so
//! comparisons in a function body do not add up. A run of
//! prefix operators (`&&&&T`, `&'a mut &'a mut T`, `*const *const T`, `!!!x`, `--x`) is
//! parsed recursively too, so each `&`, `*`, `!` or `-` in a row adds a level; `mut`, `const`
//! and lifetimes inside the run do not end it, anything else does. Other recursive syntax
//! is a `syn` limit shared by every `syn`-based macro. Closures nest even with other tokens
//! between them (`|| a || || b`), so every two `|` characters add a level until the next `;`,
//! `,` or `=>`; a match arm or condition with more than a hundred alternatives would hit it.

use proc_macro2::{Spacing, Span, TokenStream, TokenTree};
use syn::Error;
use syn::ext::IdentExt;

/// The deepest nesting a macro input may have. Far above anything written by hand, and well
/// below the depth at which `syn` overflows a compiler thread's stack.
pub const MAX_NESTING_DEPTH: usize = 128;

/// The longest identifier a macro input may have. Field, variant and path names become
/// type-level strings nested once per character (`Symbol<N, Chars<'a', Chars<…>>>`), which the
/// macros re-parse while building impls, so a long name hits the same `syn` recursion limit.
pub const MAX_IDENT_LEN: usize = 255;

/// Returns an error when `tokens` nest deeper than [`MAX_NESTING_DEPTH`] or contain an
/// identifier longer than [`MAX_IDENT_LEN`].
pub fn check_nesting_depth(tokens: &TokenStream) -> syn::Result<()> {
    match deepest(tokens.clone(), 0) {
        None => Ok(()),
        Some(Problem::TooDeep(span)) => Err(Error::new(
            span,
            format!(
                "this macro input nests more than {MAX_NESTING_DEPTH} levels of brackets or \
                 generic arguments; name an inner part with a type alias to flatten it"
            ),
        )),
        Some(Problem::LongIdent(span)) => Err(Error::new(
            span,
            format!(
                "this identifier is longer than {MAX_IDENT_LEN} characters; CGP turns names into \
                 type-level strings nested once per character, so use a shorter name"
            ),
        )),
    }
}

enum Problem {
    TooDeep(Span),
    LongIdent(Span),
}

/// Walks `tokens` starting at `depth` and returns the first token past a limit. The walk stops
/// at the depth limit, so it never recurses deeper than [`MAX_NESTING_DEPTH`] itself.
fn deepest(tokens: TokenStream, depth: usize) -> Option<Problem> {
    // One entry per open `<`: the depth already reached when it opened. Prefix runs before a
    // `<` stay counted inside it, so `&&A<&&A<…>>` adds up like `&&&&A<A<…>>`.
    let mut angles: Vec<usize> = Vec::new();
    let mut prefix = 0usize;
    // The prefix run before the current path (`&&` in `&&a::B<…>`), kept until the path's `<`.
    let mut carried = 0usize;
    // `|` characters since the last `;`, `,` or `=>`; every two are one closure (`||`, `|x|`).
    // A `||` arrives as two `|` puncts, so they are counted one by one.
    let mut pipes = 0usize;
    let mut previous: Option<(char, Spacing)> = None;

    for tree in tokens {
        let base = angles.last().copied().unwrap_or(depth);
        let current = base + prefix + pipes.div_ceil(2);
        if current > MAX_NESTING_DEPTH {
            return Some(Problem::TooDeep(tree.span()));
        }

        let mut this = None;
        match &tree {
            TokenTree::Group(group) => {
                if let Some(problem) = deepest(group.stream(), current + 1) {
                    return Some(problem);
                }
                prefix = 0;
                carried = 0;
            }
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                let joined_to_previous = matches!(previous, Some((_, Spacing::Joint)));
                match ch {
                    '<' => angles.push(base + prefix.max(carried) + 1),
                    '>' if joined_to_previous && matches!(previous, Some(('-' | '=', _))) => {}
                    '>' => {
                        angles.pop();
                    }
                    '=' if joined_to_previous && matches!(previous, Some(('<', _))) => {
                        angles.pop();
                    }
                    ';' => angles.clear(),
                    _ => {}
                }
                match ch {
                    '|' => pipes += 1,
                    ';' | ',' => pipes = 0,
                    '>' if joined_to_previous && matches!(previous, Some(('=', _))) => pipes = 0,
                    _ => {}
                }
                if ch != ':' {
                    carried = 0;
                }
                match ch {
                    '&' | '*' | '!' | '-' => prefix += 1,
                    '\'' => {}
                    _ => prefix = 0,
                }
                this = Some((ch, punct.spacing()));
            }
            TokenTree::Ident(ident) => {
                if ident.unraw().to_string().chars().count() > MAX_IDENT_LEN {
                    return Some(Problem::LongIdent(ident.span()));
                }
                let in_prefix_run =
                    ident == "mut" || ident == "const" || matches!(previous, Some(('\'', _)));
                if !in_prefix_run {
                    carried = carried.max(prefix);
                    prefix = 0;
                }
            }
            TokenTree::Literal(_) => {
                prefix = 0;
                carried = 0;
            }
        }
        previous = this;
    }

    None
}
