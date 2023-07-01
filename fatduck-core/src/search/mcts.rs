use crate::{
    chess::GameState,
    search::{SearchLimits, SearchStrategy},
    time::TimeManager,
};
use shakmaty::Move;

// If has custom params for specific formula, then will need to create own struct and fill it from
// user input.
type PlayMoveSelector = fn(&[MctsNodeData]) -> usize;
type ExploreMoveSelector = fn(&[MctsNodeData]) -> usize;

#[derive(Default, Copy, Clone)]
struct MctsParams {
    cpuct: f32,
    dirichlet_alpha: f32,
    play_selector: PlayMoveSelector,
    explore_selector: ExploreMoveSelector,
}

impl MctsParams {
    pub fn new(
        cpuct: f32,
        dirichlet_alpha: f32,
        play_selector: PlayMoveSelector,
        explore_selector: ExploreMoveSelector,
    ) -> Self {
        Self {
            cpuct,
            dirichlet_alpha,
            play_selector,
            explore_selector,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct MctsNodeData {
    /// Q = W - L
    q: f32,
    /// Total visit count
    n: f32,
    /// Predicted draw score
    d: f32,
    /// Prior probability of selecting this node (policy)
    p: f32,
}

#[derive(Clone, Copy, Default)]
struct MctsEdgeData {
    q: f32,
    n: f32,
    d: f32,
    p: f32,
}

struct Mcts {
    params: MctsParams,
}

impl Mcts {
    pub fn new(params: MctsParams) -> Self {
        Self { params }
    }
}

impl<TM: TimeManager> SearchStrategy<TM> for Mcts {
    type NodeData = MctsNodeData;
    type EdgeData = MctsEdgeData;
    type Params = MctsParams;
    type Stats = ();

    fn name(&self) -> &str {
        "MCTS"
    }

    fn dynamic_time_search(
        &mut self,
        state: &GameState,
        time_manager: &TM,
        params: &mut Self::Params,
    ) -> Move {
        todo!();
    }

    fn fixed_limit_search(
        &mut self,
        state: &GameState,
        limits: SearchLimits,
        params: &mut Self::Params,
    ) -> Move {
        todo!()
    }

    fn parameters(&self) -> &Self::Params {
        todo!();
    }

    fn all_stats(&self) -> &Self::Stats {
        todo!();
    }
}

/// Selection of which child node of root to play:
/// struct MaxNodes;
/// impl PlayMoveSelection for MaxNodes {
///   type AlgoParams = ();
///
///   fn best_move_index(&self, params: &Self::AlgoParams, root_data: MctsNodeData) -> usize {
///     // Iterate over all children from root and
///   }
///
///   fn best_move(&self, params: &Self::AlgoParams, move_data: MctsNodeData) -> Move {
///
/// }
///
trait PlayMoveSelection {
    // includes formula specific parameters and any info needed from search
    type AlgoParams;

    fn best_move_index(&self, params: &Self::AlgoParams, root_data: MctsNodeData) -> usize;
    fn best_move(&self, params: &Self::AlgoParams, root_data: MctsNodeData) -> Move;
}

// Selection of which move to explore
trait ExploreMoveSelection {
    type AlgoParams;

    fn best_explore_move(&self, params: &Self::AlgoParams, move_data: MctsNodeData) -> Move;
    fn best_explore_move_index(&self, params: &Self::AlgoParams, move_data: MctsNodeData) -> usize;
    fn add_explore_scores_to_nodes(
        &self,
        params: &Self::AlgoParams,
        move_data: &mut MctsNodeData,
        move_data: &mut MctsEdgeData,
    );
}

trait ResultPropagation {
    type AlgoParams;

    fn propagate_result(
        &self,
        params: &Self::AlgoParams,
        move_data: &mut MctsNodeData,
        move_data: &mut MctsEdgeData,
    );
}
