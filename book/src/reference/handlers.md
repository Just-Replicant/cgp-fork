# Handlers

Import the unified names with `use cgp_fork::handler::prelude::*;`. They are
not in `cgp_fork::prelude`, which still exports the 0.10 `Handler`.

| 0.10 | Unified |
|---|---|
| `Computer<Ctx, Code, Input>` | `Handler<Ctx, Code, Input>` |
| `ComputerRef<Ctx, Code, Input>` | `Handler<Ctx, Code, &'a Input>` |
| `TryComputer<Ctx, Code, Input>` | `Handler<Ctx, Code, Input, Output = Result<T, Ctx::Error>>` |
| `TryComputerRef` | `Handler<Ctx, Code, &'a Input, Output = Result<…>>` |
| `AsyncComputer` | `AsyncHandler<Ctx, Code, Input>` |
| `AsyncComputerRef` | `AsyncHandler<Ctx, Code, &'a Input>` |
| `Handler` (async, fallible) | `AsyncHandler<…, Output = Result<T, Ctx::Error>>` |
| `HandlerRef` | `AsyncHandler<Ctx, Code, &'a Input, Output = Result<…>>` |
| `Producer<Ctx, Code>` | `Handler<Ctx, Code, ()>` |

`Promote`, `PromoteOk`, `PromoteDeref`, and `IgnoreInput` replace the old
promote modules. `Pipe` and `TryPipe` compose handlers. `MatchWith<Handlers, All | First, Owned | Ref | Mut>`
replaces the six dispatch providers.

The prelude exports the provider traits and not `CanHandle` or `CanHandleAsync`.
Both pairs name their method `handle` or `handle_async`, so a call such as
`Pipe::<H>::handle(...)` would be ambiguous with both in scope.

`compute`, `try_compute`, `produce`, and `send_run` are the old method names.
The new methods are `handle` and `handle_async`. The [migration](../migration.md)
table is the list the codemod rewrites.
