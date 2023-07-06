mod graph;
mod mcts;
pub mod stats;

use std::time::Duration;

use crate::{
    chess::GameState,
    time::TimeManager,
};
use shakmaty::Move;

#[derive(Default)]
struct SearchManager<S: SearchStrategy, T: TimeManager> {
    search: S,
    time_manager: T,
    root_state: GameState,
    game_tree: Tree<S::NodeData, S::EdgeData>,
    limits: SearchLimits,
}

impl<S: SearchStrategy, T: TimeManager> SearchManager<S, T> {
    pub fn new(
        search: T,
        root_state: GameState,
        game_tree: Tree<S::NodeData, S::EdgeData>,
        time_manager: T,
        limits: SearchLimits,
    ) -> Self {
        Self {
            search,
            time_manager,
            root_state,
            game_tree,
            limits,
        }
    }

    // Returns best move found within `Self.limits`
    pub fn make_best_move(&mut self) -> Move {
        todo!();
    }

    /// Returns best move found within `limits`.
    pub fn make_best_move_with_limits(&mut self, limits: SearchLimits) -> Move {
        todo!();
    }

    pub fn analysis(&self) -> String {
        todo!();
    }

    pub fn iteration_stats(&self) -> String {
        todo!();
    }

    pub fn stop_search(&mut self) {
        todo!();
    }
}

pub trait SearchStrategy {
    type NodeData: Copy + Send + Sync;
    type EdgeData: Copy + Send + Sync;
    type Params: Default + Copy + Send + Sync;
    type Stats: Copy + Clone;

    /// Name of the search strategy (e.g "Mcts", "AlphaBeta")
    fn name(&self) -> &str;
    fn fixed_limit_search(
        &mut self,
        state: &GameState,
        limits: SearchLimits,
        params: &mut Self::Params,
    ) -> Move;
    fn params(&self) -> &Self::Params;
    fn all_stats(&self) -> &Self::Stats;
}

#[derive(Clone, Copy, Debug)]
pub enum SearchLimits {
    Time(Duration),
    Nodes(usize),
    Depth(usize),
    Infinite,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self::Time(std::time::Duration::from_millis(1_000))
    }
}
