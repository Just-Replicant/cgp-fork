# Security policy

## Supported versions

Only the latest released `0.x` minor version of `cgp-fork` and `cgp-fork-macro` receives fixes.

## Reporting a vulnerability

Please report security issues **privately** through GitHub's
[private vulnerability reporting](https://github.com/Just-Replicant/cgp-fork/security/advisories/new)
rather than a public issue. Include the affected version, a minimal reproduction, and the impact.
You should get an acknowledgement within a week.

`cgp-fork` is a compile-time library: its procedural macros run inside your build, and the facade
contains no `unsafe` code. Reports about generated code that is unsound, a macro that can be made
to emit `unsafe` the user did not write, or a dependency with a RustSec advisory are all in scope.

## Supply chain

CI pins every GitHub Action to a commit SHA, installs tools from checksum-verified downloads or
`cargo install --locked`, and runs `cargo deny` and `cargo audit` on every push and weekly.
Releases are published with crates.io trusted publishing (OIDC); no long-lived registry token exists.
