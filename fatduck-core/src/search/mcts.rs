use crate::{
    chess::GameState,
    search::{SearchLimits, SearchStrategy},
    time::TimeManager,
};

use shakmaty::Move;
/// The general API for a Monte Carlo Tree Search algorithm.

// Intended usage:
// ```
// let params = MctsParams::default();
// let play_selector = MaxVisits::default();
// let explore_selector = Puct::default();
// let rp = Minimax::default();
// let nvr = QIncremental::default();
//
// let mut mcts = Mcts::new(params, play_selector, explore_selector, rp, nvr);
// // or
// // Strongest configuration
// let mut mcts = Mcts::default();
// ```
struct Mcts<PS, ES, RP, NVR>
where
    PS: PlaySelector,
    ES: ExploreSelector,
    RP: ResultPropagator,
    NVR: NodeValueRecalculator,
{
    params: MctsParams,
    play_selector: PS,
    explore_selector: ES,
    result_propagator: RP,
    node_value_recalculator: NVR,
}

impl<PS, ES, RP, NVR> Mcts<PS, ES, RP, NVR>
where
    PS: PlaySelector,
    ES: ExploreSelector,
    RP: ResultPropagator,
    NVR: NodeValueRecalculator,
{
    pub fn new(
        params: MctsParams,
        play_selector: PS,
        explore_selector: ES,
        result_propagator: RP,
        node_value_recalculator: NVR,
    ) -> Self {
        Self {
            params,
            play_selector,
            explore_selector,
            result_propagator,
            node_value_recalculator,
        }
    }
}

impl<PS, ES, RP, NVR> SearchStrategy for Mcts<PS, ES, RP, NVR>
where
    PS: PlaySelector,
    ES: ExploreSelector,
    RP: ResultPropagator,
    NVR: NodeValueRecalculator,
{
    type NodeData = MctsNodeData;
    type EdgeData = MctsEdgeData;
    type Params = MctsParams;
    type Stats = MctsStats;

    fn name(&self) -> &str {
        "Mcts"
    }

    fn fixed_limit_search(
        &mut self,
        state: &GameState,
        limits: SearchLimits,
        params: &mut Self::Params,
    ) -> Move {
        todo!();
    }

    fn params(&self) -> &Self::Params {
        todo!()
    }

    fn all_stats(&self) -> &Self::Stats {
        todo!()
    }
}

#[derive(Copy, Clone)]
struct MctsParams {
    cpuct: f32,
    dirichlet_alpha: f32,
}

impl MctsParams {
    pub fn new(cpuct: f32, dirichlet_alpha: f32) -> Self {
        Self {
            cpuct,
            dirichlet_alpha,
        }
    }
}

impl Default for MctsParams {
    fn default() -> Self {
        Self::new(1.0, 0.3)
    }
}

#[derive(Clone, Copy)]
struct MctsStats {
    nodes: u32,
}

#[derive(Clone, Copy)]
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

trait PlaySelector {
    type AlgoParams;

    fn best_move_idx(&self, params: &Self::AlgoParams, root_data: MctsNodeData) -> usize;
    fn best_move(&self, params: &Self::AlgoParams, root_data: MctsNodeData) -> Move;
}

trait ExploreSelector {
    type AlgoParams;

    fn best_move(&self, params: &Self::AlgoParams, move_data: MctsNodeData) -> Move;
    fn best_move_idx(&self, params: &Self::AlgoParams, move_data: MctsNodeData) -> usize;
    fn add_explore_scores_to_nodes(
        &self,
        params: &Self::AlgoParams,
        move_data: &mut MctsNodeData,
        move_data: &mut MctsEdgeData,
    );
}

trait NodeValueRecalculator {
    type AlgoParams;

    fn recalculate_node_value(
        &self,
        params: &Self::AlgoParams,
        move_data: &mut MctsNodeData,
        move_data: &mut MctsEdgeData,
    );
}

trait ResultPropagator {
    type AlgoParams;

    fn propagate_result(
        &self,
        params: &Self::AlgoParams,
        move_data: &mut MctsNodeData,
        move_data: &mut MctsEdgeData,
    );
}
