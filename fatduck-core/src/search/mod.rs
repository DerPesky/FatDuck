mod graph;
mod mcts;

use crate::{chess::GameState, time::TimeManager};
use shakmaty::Move;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum SearchLimits {
    Time(Duration),
    Nodes(usize),
    Depth(usize),
    Infinite,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self::Time(Duration::from_millis(1_000))
    }
}

// my_cool_project.rs
// ----------------------------------------------------------------------------------------
// let strat = AlphaBeta::new(mctsParams);
// let starting_board = GameState::default();
// let time_manager = AlphaZeroTM::default();
// let limits = SearchLimits::default(); // searches for 1 second by default
// let mcts_manager = SearchManager::new(strat, starting_board, time_manager, limits);
//
// loop {
//   // searches for 1 second then sends move
//   let best_move: Move = mcts_manager.make_best_move();
//   Uci::send_move(best_move);
// }
#[derive(Default)]
struct SearchManager<T: SearchStrategy<TM>, TM: TimeManager> {
    strategy: T,
    root_state: GameState,
    // tree: Dag<T::NodeData, T::EdgeData>,
    time_manager: TM,
    limits: SearchLimits,
}

impl<T: SearchStrategy<TM>, TM: TimeManager> SearchManager<T, TM> {
    pub fn new(strategy: T, root_state: GameState, time_manager: TM, limits: SearchLimits) -> Self {
        Self {
            strategy,
            root_state,
            time_manager,
            limits,
        }
    }

    // Returns best move found within `SearchLimits`
    pub fn make_best_move(&mut self) -> Move {
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

// User creates search strategy object, calls search with given state, time_manager, and limits
pub trait SearchStrategy<TM: TimeManager> {
    type NodeData: Default + Copy + Clone + Send + Sync;
    type EdgeData: Default + Copy + Clone + Send + Sync;
    type Params: Default + Copy + Clone + Send + Sync;
    type Stats: Default + Copy + Clone;

    fn name(&self) -> &str;
    // Returns the best move given a state and time manager which can dynamically adjust the time
    // limit.
    fn dynamic_time_search(
        &mut self,
        state: &GameState,
        time_manager: &TM,
        params: &mut Self::Params,
    ) -> Move;
    // Returns the best move given a state and a fixed search limit.
    fn fixed_limit_search(
        &mut self,
        state: &GameState,
        limits: SearchLimits,
        params: &mut Self::Params,
    ) -> Move;
    fn parameters(&self) -> &Self::Params;
    // fn iteration_stats(&self) -> &IterationStats;
    fn all_stats(&self) -> &Self::Stats;
}
