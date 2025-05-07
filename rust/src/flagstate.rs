
/**
Is a use flag set or not set.
 */
#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum FlagState {
    Unset = 0,
    Set = 1,
}
