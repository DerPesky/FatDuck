use crate::search::{IterationStats, SearchLimits};

/// Responsible for deciding when to stop the search based off of `SearchStats` and `SearchLimits`.
/// Each `TimeManager` implementation will have the ability to work off of the common base stats in
/// `SearchStats` but can also specialize for a particular `SearchStats` implementation.
pub trait TimeManager: Default + Sized {
    fn should_stop(&self, search_info: &IterationStats, search_limits: &SearchLimits) -> bool;
    fn adjust_time_limit(&self, search_info: &IterationStats, search_limits: &mut SearchLimits);
}

pub(crate) enum TimeUsageHint {
    Normal,
    NeedMoreTime,
    ImmediateMove,
}
