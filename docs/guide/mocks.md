# Mocks

`#[cgp_component(Greeter, mock)]` generates two types when the `mock` feature is on:

- `GreeterMock`, the state. Store it on the test context in a field named `greeter_mock`
  and derive `HasField`.
- `MockGreeter`, the provider. Wire `GreeterComponent: MockGreeter`.

`greet_returns` queues one return value. `greet_with` installs a closure that wins
over the queue. `greet_call_count` is how many times the method ran. A call with
neither a value nor a closure panics.

The methods take `&self` or `&mut self` and return an owned value. The macro rejects
async, unsafe, and generic methods, associated items, references, `impl Trait`, and
`Self`. It also rejects a `where` clause on the trait. A `where` clause on a method
is kept, and `Self` in it is the context, the same way the provider trait rewrites it.

The closure runs without the handler cell borrowed, so it may call the same method.
That inner call does not see the closure (it was taken out) and uses the queue.
A `greet_with` from inside the closure replaces it. The previous closure is put
back when the call returns, including when it panics, unless a replacement is
already installed.

```rust,ignore
#[cgp_component(Greeter, mock)]
pub trait CanGreet {
    fn greet(&self, name: &str) -> String;
}
```

Build the example with `cargo run -p cgp-fork --example mock --features mock`.
The test is `tests/capabilities/mock_component.rs`, and it runs with `--features mock`
or `--all-features`.
