---
name: Bug report
about: A macro rejects valid code, accepts invalid code, panics, or generates code that misbehaves
labels: bug
---

**What happened**

<!-- The error, panic, or wrong behaviour. Paste the full compiler output. -->

**Minimal reproduction**

```rust
use cgp_fork::prelude::*;

// The smallest code that shows the problem.
```

**Expected**

**Environment**

- `cgp-fork` version:
- `rustc --version`:
- Enabled features:
- Output of `cargo cgp check`, if you use [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp):
