//! Robustness target: every macro entrypoint must turn malformed input into a
//! `syn::Error`, never a panic, because a proc-macro panic surfaces in the user's
//! build as an opaque "proc macro panicked" with no span.
//!
//! This is a dependency-free, deterministic fuzzer. It builds inputs two ways:
//! random token soup from a CGP-flavoured vocabulary, and token-tree mutations
//! (delete, duplicate, swap, truncate) of valid seed inputs, recursing into groups.
//! Each input is fed to every entrypoint under `catch_unwind`.
//!
//! `CGP_FUZZ_ITERS` (default 300) sets the iterations per entrypoint, and
//! `CGP_FUZZ_SEED` (default fixed) the PRNG seed, so a failure is reproducible:
//! `CGP_FUZZ_ITERS=20000 cargo test -p cgp-fork-macro robustness -- --nocapture`.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

/// A macro entrypoint by name. Shared with the cargo-fuzz targets in `fuzz/`,
/// which include this file, so both fuzzers cover the same list.
pub type Entrypoint = (
    &'static str,
    fn(TokenStream, TokenStream) -> syn::Result<TokenStream>,
);

macro_rules! body_only {
    ($path:path) => {{
        fn call(_attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
            $path(body)
        }
        call as fn(TokenStream, TokenStream) -> syn::Result<TokenStream>
    }};
}

/// Every macro entrypoint, as `(attr, body)` functions.
pub fn entrypoints() -> Vec<Entrypoint> {
    use crate::{extra_macro_lib as extra, macro_lib as lib};
    vec![
        ("async_trait", crate::async_impl::impl_async),
        ("blanket_trait", lib::blanket_trait),
        ("cgp_auto_error", lib::cgp_auto_error),
        ("cgp_auto_getter", lib::cgp_auto_getter),
        ("cgp_auto_impl", lib::cgp_auto_impl),
        ("cgp_component", lib::cgp_component),
        ("cgp_fn", lib::cgp_fn),
        ("cgp_getter", lib::cgp_getter),
        ("cgp_impl", lib::cgp_impl),
        ("cgp_new_provider", lib::cgp_new_provider),
        ("cgp_provider", lib::cgp_provider),
        ("cgp_type", lib::cgp_type),
        ("derive_provider", lib::derive_provider),
        ("cgp_computer", extra::cgp_computer),
        ("cgp_producer", extra::cgp_producer),
        ("cgp_auto_dispatch", extra::cgp_auto_dispatch),
        ("cgp_auto_log", extra::cgp_auto_log),
        ("delegate_components", body_only!(lib::delegate_components)),
        (
            "delegate_and_check_components",
            body_only!(lib::delegate_and_check_components),
        ),
        ("check_components", body_only!(lib::check_components)),
        ("cgp_for_each", body_only!(lib::cgp_for_each)),
        ("cgp_preset", body_only!(lib::cgp_preset)),
        ("cgp_namespace", body_only!(lib::cgp_namespace)),
        ("Symbol", body_only!(lib::Symbol)),
        ("Path", body_only!(lib::Path)),
        ("Product", body_only!(lib::Product)),
        ("product", body_only!(lib::product)),
        ("Sum", body_only!(lib::Sum)),
        ("Struct", body_only!(lib::Struct)),
        ("Enum", body_only!(lib::Enum)),
        ("derive_has_field", body_only!(lib::derive_has_field)),
        ("derive_has_fields", body_only!(lib::derive_has_fields)),
        ("derive_build_field", body_only!(lib::derive_build_field)),
        (
            "derive_extract_field",
            body_only!(lib::derive_extract_field),
        ),
        ("derive_from_variant", body_only!(lib::derive_from_variant)),
        ("derive_cgp_data", body_only!(lib::derive_cgp_data)),
        ("derive_cgp_record", body_only!(lib::derive_cgp_record)),
        ("derive_cgp_variant", body_only!(lib::derive_cgp_variant)),
    ]
}

/// Valid (or nearly valid) inputs the mutator starts from: `(attr, body)`.
pub const SEEDS: &[(&str, &str)] = &[
    (
        "",
        "App { open ErrorRaiserComponent; GreeterComponent: GreetHello, @ErrorRaiserComponent.String: DisplayError, [FooComponent, BarComponent]: UseDelegate<new Table { u32: A, String: B }>, }",
    ),
    (
        "Greeter",
        "pub trait CanGreet<T>: HasName where T: Clone { type Out; fn greet(&self, value: &T) -> Self::Out; async fn run(&mut self) -> u32 { 1 } }",
    ),
    (
        "new GreetHello",
        "impl<T> Greeter<T> where Self: HasName { fn greet(&self, #[implicit] name: &str) -> String { format!(\"{name}\") } }",
    ),
    (
        "GreeterComponent",
        "impl<Context, T: Clone> Greeter<Context, T> for GreetHello where Context: HasName { fn greet(context: &Context) -> String { String::new() } }",
    ),
    ("", "\"hello world\""),
    ("", "u32, String, Vec<u8>, &'a str"),
    (
        "",
        "pub struct Person<'a, T: Clone> { pub name: &'a str, pub value: T, } ",
    ),
    (
        "",
        "pub enum Shape<T> { Circle(T), Square { side: T }, Empty }",
    ),
    (
        "",
        "@cgp.core.error in DefaultNamespace { ErrorRaiserComponent, FooComponent: Bar, }",
    ),
    (
        "",
        "App { GreeterComponent, [FooComponent, BarComponent]: (u32, String), }",
    ),
    ("", "MyPreset { GreeterComponent: GreetHello, } "),
    (
        "HasName",
        "pub trait HasName { fn name(&self) -> &str; fn age(&mut self) -> &mut u8; }",
    ),
    (
        "",
        "impl App { fn raise_error(e: String) -> Error { todo!() } }",
    ),
];

const VOCABULARY: &[&str] = &[
    "App",
    "Self",
    "self",
    "T",
    "Context",
    "__Context__",
    "u32",
    "String",
    "\"s\"",
    "'a",
    "0",
    "1",
    "pub",
    "fn",
    "trait",
    "impl",
    "struct",
    "enum",
    "type",
    "where",
    "for",
    "in",
    "async",
    "new",
    "open",
    "namespace",
    "use",
    "mut",
    "unsafe",
    "const",
    "dyn",
    "->",
    "=>",
    ":",
    "::",
    ";",
    ",",
    ".",
    "@",
    "#",
    "!",
    "?",
    "&",
    "=",
    "+",
    "<",
    ">",
    "*",
    "|",
    "{}",
    "()",
    "[]",
    "{ A: B }",
    "(u32, T)",
    "<T>",
    "[implicit]",
    "#[implicit]",
    "UseDelegate",
    "Symbol",
    "Greeter",
    "GreeterComponent",
    "IsProviderFor",
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
}

fn soup(rng: &mut Rng) -> TokenStream {
    let len = rng.below(24);
    let text: Vec<&str> = (0..len)
        .map(|_| VOCABULARY[rng.below(VOCABULARY.len())])
        .collect();
    TokenStream::from_str(&text.join(" ")).unwrap_or_default()
}

fn mutate(rng: &mut Rng, stream: TokenStream, depth: usize) -> TokenStream {
    let mut trees: Vec<TokenTree> = stream.into_iter().collect();
    let edits = 1 + rng.below(3);
    for _ in 0..edits {
        if trees.is_empty() {
            break;
        }
        let index = rng.below(trees.len());
        match rng.below(6) {
            0 => {
                trees.remove(index);
            }
            1 => {
                let tree = trees[index].clone();
                trees.insert(index, tree);
            }
            2 => {
                let other = rng.below(trees.len());
                trees.swap(index, other);
            }
            3 => trees.truncate(index),
            4 => {
                if let Ok(fragment) = TokenStream::from_str(VOCABULARY[rng.below(VOCABULARY.len())])
                {
                    for (offset, tree) in fragment.into_iter().enumerate() {
                        trees.insert(index + offset, tree);
                    }
                }
            }
            _ => {
                if depth < 4
                    && let TokenTree::Group(group) = &trees[index]
                {
                    let delimiter = if rng.below(4) == 0 {
                        [Delimiter::Brace, Delimiter::Parenthesis, Delimiter::Bracket][rng.below(3)]
                    } else {
                        group.delimiter()
                    };
                    let inner = mutate(rng, group.stream(), depth + 1);
                    trees[index] = TokenTree::Group(Group::new(delimiter, inner));
                }
            }
        }
    }
    trees.into_iter().collect()
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

#[test]
fn entrypoints_never_panic() {
    let iterations = env_u64("CGP_FUZZ_ITERS", 300);
    let seed = env_u64("CGP_FUZZ_SEED", 0x00C6_9F0C_0FFE_E123);

    // Do not replace the process panic hook. `cargo test` runs this next to the
    // other unit tests, and the hook is process-global: swapping it hides their
    // panic messages, and a panic before the old hook is restored leaves the
    // silence in place. `catch_unwind` still records the failing input. A panic
    // also prints through the test harness hook, which is the backtrace we want.
    let mut failures = Vec::new();
    for (name, entrypoint) in entrypoints() {
        let mut rng = Rng(seed ^ name.len() as u64 ^ (name.as_bytes()[0] as u64) << 32);
        for iteration in 0..iterations {
            let (attr, body) = if rng.below(3) == 0 {
                (soup(&mut rng), soup(&mut rng))
            } else {
                let (attr, body) = SEEDS[rng.below(SEEDS.len())];
                let attr = TokenStream::from_str(attr).unwrap();
                let body = TokenStream::from_str(body).unwrap();
                if rng.below(2) == 0 {
                    (mutate(&mut rng, attr, 0), body)
                } else {
                    (attr, mutate(&mut rng, body, 0))
                }
            };
            let (attr_text, body_text) = (attr.to_string(), body.to_string());
            let outcome = catch_unwind(AssertUnwindSafe(|| entrypoint(attr, body)));
            if outcome.is_err() {
                failures.push(format!(
                    "{name} panicked (iteration {iteration}):\n  attr: {attr_text}\n  body: {body_text}"
                ));
                break;
            }
        }
    }

    assert!(
        failures.is_empty(),
        "macro entrypoints panicked:\n{}",
        failures.join("\n")
    );
}
