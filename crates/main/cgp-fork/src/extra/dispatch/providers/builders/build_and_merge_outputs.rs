use crate::core::field::traits::MapFields;
use crate::core::prelude::*;
use crate::extra::dispatch::{BuildAndMerge, BuildWithHandlers};
use crate::extra::handler::{
    ComputerComponent, ComputerRefComponent, HandlerComponent, HandlerRefComponent,
    TryComputerComponent, TryComputerRefComponent,
};

/// Builds `Output` by merging the result of each handler in `Handlers`.
pub struct BuildAndMergeOutputs<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

delegate_components! {
    <Output, Handlers: MapFields<ToBuildAndMergeHandler>>
    BuildAndMergeOutputs<Output, Handlers> {
        [
            ComputerComponent,
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ]:
            BuildWithHandlers<Output, Handlers::Mapped>
    }
}

/// [`MapType`](crate::core::field::traits::MapType) that wraps each handler in [`BuildAndMerge`](crate::extra::dispatch::BuildAndMerge).
pub struct ToBuildAndMergeHandler;

impl MapType for ToBuildAndMergeHandler {
    type Map<Handler> = BuildAndMerge<Handler>;
}
