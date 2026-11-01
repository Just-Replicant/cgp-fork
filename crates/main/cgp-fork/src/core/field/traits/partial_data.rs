/// A partial builder and the finished value it constructs.
pub trait PartialData {
    /// The value produced once every required field is present.
    type Target;
}
