use crate::macro_core::export_constructs;

export_constructs! {
    UnifiedHandler => unified::Handler,
    UnifiedHandlerComponent => unified::HandlerComponent,
    UnifiedAsyncHandler => unified::AsyncHandler,
    UnifiedAsyncHandlerComponent => unified::AsyncHandlerComponent,
    UnifiedPromote => unified::Promote,
    UnifiedMatchValues => unified::MatchValues,
    UnifiedFirst => unified::First,
    UnifiedAll => unified::All,
    Compat,
    CgpCompat010 => __cgp_compat_0_10,

    Computer,
    AsyncComputer,
    Producer,

    ComputerComponent,
    ComputerRefComponent,
    TryComputerComponent,
    TryComputerRefComponent,
    AsyncComputerComponent,
    AsyncComputerRefComponent,
    HandlerComponent,
    HandlerRefComponent,
    ProducerComponent,

    PromoteComputer,
    PromoteTryComputer,
    PromoteAsyncComputer,
    PromoteHandler,
    PromoteProducer,
}
