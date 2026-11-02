# Async

`#[cgp_async]` desugars `async fn` in a trait to `impl Future`. `#[cgp_async(Send)]`
adds `+ Send` on that future. The future still borrows `&self`, so it is not
`'static`.

`#[async_trait]` is the old name. Under `#[cgp_component]` or `#[cgp_fn]` it is
rewritten, and the warning points at the attribute you wrote.

```rust
use cgp_fork::prelude::*;

#[cgp_component(Greeter)]
#[cgp_async]
pub trait CanGreet {
    async fn greet(&self) -> &'static str;
}

pub struct GreetHello;

#[cgp_provider(GreeterComponent)]
#[cgp_async]
impl<Context> Greeter<Context> for GreetHello {
    async fn greet(_context: &Context) -> &'static str {
        "hello"
    }
}

pub struct Person;

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let greeting = Person.greet();
    let _ = greeting;
}
```

`CanRun` is the runner. Its future is `Send` only when the component it runs was
declared with `#[cgp_async(Send)]`. `CanSendRun` is deprecated.

`AsyncRuntime` (spawn, sleep, timeout) is implemented for a context that stores
a runtime. The `tokio` and `smol` features supply the adapters. Sleep and timeout
are `Send + 'static` futures: the wait starts when the future is first polled,
and dropping it cancels the wait. The runnable example is
`crates/main/cgp-fork/examples/runtime.rs`.
