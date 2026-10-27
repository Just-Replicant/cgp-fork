/**
   The `IsProviderFor` trait is used to propagate the constraints required to
   implement the provider trait that corresponds to the `Component` type.

   ## Parameters

   The `IsProviderFor` trait parameters are used as follows:

   - `Component`: The component name type that corresponds to the provider trait.
   - `Context`: The `Context` type used in the provider trait.
   - `Params`: Any additional generic parameters in the provider trait, with
     multiple generic parameters grouped inside a tuple.

   ## `IsProviderFor` as a constraint carrier

   The trait definition for `IsProviderFor` has an empty body that can be trivially
   implemented. However, when used with `#[cgp_provider]` or `#[cgp_new_provider]`,
   on a provider trait implementation, it would be implemented by the `Provider` type
   with the same set of constraints as the provider trait implementation.

   The `IsProviderFor` trait is included as a supertrait of all CGP provider traits
   generated from `#[cgp_component]`. This means that when there is any unsatisfied
   constraint in the provider trait implementation, it would also result in the
   same error shown in the `IsProviderFor` implementation.

   ## Why is this trait necessary?

   The trait is necessary to force the Rust compiler to show any relevant error message
   when there are unsatisfied constraints in the provider trait implementation. By default,
   Rust would hide the error messages from the provider trait implementation, because there
   is also an alternative candidate implementation available, which is the blanket
   implementation of the provider trait.

   On the other hand, the `IsProviderFor` trait is explicitly propagated inside
   `delegate_components!`, together with `DelegateComponent`. Because of this different
   implementation path, we are able to "unhide" the error messages that were hidden away
   by Rust.

   ## Example Definition

   Given a CGP trait definition such as:

   ```rust
   # use core::marker::PhantomData;
   # use cgp_fork::prelude::*;
   #[cgp_component(FooGetterAt)]
   pub trait CanGetFooAt<I, J> {
       fn foo_at(&self, _phantom: PhantomData<(I, J)>) -> u64;
   }
   ```

   The following provider trait would be generated with the `IsProviderFor` supertrait
   (illustrative, not compiled):

   ```rust,ignore
   pub trait FooGetterAt<Context, I, J>:
       IsProviderFor<FooGetterAtComponent, Context, (I, J)>
   {
       fn foo_at(context: &Context, _phantom: PhantomData<(I, J)>) -> u64;
   }
   ```

   ## Example Implementation

   Given a provider trait implementation such as:

   ```rust
   # use core::marker::PhantomData;
   # use cgp_fork::prelude::*;
   # #[cgp_component(FooGetterAt)]
   # pub trait CanGetFooAt<I, J> {
   #     fn foo_at(&self, _phantom: PhantomData<(I, J)>) -> u64;
   # }
   pub struct GetFooValue;

   #[cgp_provider(FooGetterAtComponent)]
   impl<Context, I, J> FooGetterAt<Context, I, J> for GetFooValue
   where
       Context: HasField<Symbol!("foo"), Value = u64>,
   {
       fn foo_at(context: &Context, _phantom: PhantomData<(I, J)>) -> u64 {
           *context.get_field(PhantomData)
       }
   }
   ```

   The following implementation for `IsProviderFor` would be generated (illustrative, not compiled):

   ```rust,ignore
   impl<Context, I, J>
       IsProviderFor<FooGetterAtComponent, Context, (I, J)>
       for GetFooValue
   where
       Context: HasField<Symbol!("foo"), Value = u64>,
   {
   }
   ```

   ## Example Delegation

   Given a component delegation such as:

   ```rust
   # use core::marker::PhantomData;
   # use cgp_fork::prelude::*;
   # #[cgp_component(FooGetterAt)]
   # pub trait CanGetFooAt<I, J> {
   #     fn foo_at(&self, _phantom: PhantomData<(I, J)>) -> u64;
   # }
   # pub struct GetFooValue;
   # #[cgp_provider(FooGetterAtComponent)]
   # impl<Context, I, J> FooGetterAt<Context, I, J> for GetFooValue
   # where
   #     Context: HasField<Symbol!("foo"), Value = u64>,
   # {
   #     fn foo_at(context: &Context, _phantom: PhantomData<(I, J)>) -> u64 {
   #         *context.get_field(PhantomData)
   #     }
   # }
   pub struct MyAppComponents;

   delegate_components! {
       MyAppComponents {
           FooGetterAtComponent: GetFooValue,
       }
   }

   #[derive(HasField)]
   pub struct App {
       pub foo: u64,
   }

   // `MyAppComponents` now has `IsProviderFor<FooGetterAtComponent, App, (I, J)>`, so it is
   // usable as a `FooGetterAt` provider for `App`:
   let value = <MyAppComponents as FooGetterAt<App, u8, u16>>::foo_at(&App { foo: 3 }, PhantomData);
   assert_eq!(value, 3);
   ```

   The following `IsProviderFor` implementation would be generated (illustrative, not compiled):

   ```rust,ignore
   impl<Context, Params>
       IsProviderFor<FooGetterAtComponent, Context, Params>
       for MyAppComponents
   where
       GetFooValue: IsProviderFor<FooGetterAtComponent, Context, Params>,
   {
   }
   ```

   This means that `MyAppComponents` has an explicit implementation of `IsProviderFor`
   for all possible `Context` and `Params`, with the `where` constraint propagating
   the constraints coming from `GetFooValue` with the same `Context` and `Params`.

   Because of this is an explicit implementation and not a blanket implementation,
   Rust would follow the implementation path and surface all unsatisfied constraints
   from `GetFooValue`.
*/
pub trait IsProviderFor<Component, Context, Params: ?Sized = ()> {}
