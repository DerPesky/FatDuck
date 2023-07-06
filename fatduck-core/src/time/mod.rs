use crate::{
    chess::uci::UciGoParams,
    search::{stats::IterationStats, SearchLimits},
};

/// Responsible for deciding when to stop the search based off of `SearchStats` and `SearchLimits`.
/// Each `TimeManager` implementation will have the ability to work off of the common base stats in
/// `SearchStats` but can also specialize for a particular `SearchStats` implementation.
pub trait TimeManager: Default {
    fn stopper(&self, params: UciGoParams, tree: Tree) -> Box<dyn SearchStopper>;
}

trait SearchStopper: Send + Sync {
    fn should_stop(&self, search_info: &IterationStats, search_limits: &SearchLimits) -> bool;
    fn on_search_done(&self, search_info: &IterationStats);
}

pub(crate) enum TimeUsageHint {
    Normal,
    NeedMoreTime,
    ImmediateMove,
}

#[derive(Default)]
struct StoppersHints {
    remaining_time: u64,
    remaining_playouts: u64,
    estimated_nps: Option<f32>,
}

impl StoppersHints {
    pub fn new(remaining_time: u64, remaining_playouts: u64, estimated_nps: Option<f32>) -> Self {
        Self {
            remaining_time,
            remaining_playouts,
            estimated_nps,
        }
    }

    pub fn remaining_time_ms(&self) -> u64 {
        self.remaining_time
    }

    pub fn remaining_time_mut(&mut self) -> &mut u64 {
        &mut self.remaining_time
    }

    pub fn remaining_playouts(&self) -> u64 {
        self.remaining_playouts
    }

    pub fn remaining_playouts_mut(&mut self) -> &mut u64 {
        &mut self.remaining_playouts
    }

    pub fn estimated_nps(&self) -> Option<f32> {
        self.estimated_nps
    }

    pub fn estimated_nps_mut(&mut self) -> &mut Option<f32> {
        &mut self.estimated_nps
    }
}
