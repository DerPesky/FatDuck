use std::time::Duration;

use shakmaty::{Color, Move, Outcome};

use crate::time::TimeUsageHint;

pub(crate) struct IterationStats {
    time_since_movestart: Duration,
    time_since_first_batch: Duration,
    total_nodes: usize,
    nodes_since_movestart: usize,
    batches_since_movestart: usize,
    avg_depth: i32,
    edge_n: Vec<usize>,
    win_found: bool,
    may_resign: bool,
    num_losing_edges: usize,
    time_usage_hint: TimeUsageHint,
}

impl Default for IterationStats {
    fn default() -> Self {
        Self {
            time_since_movestart: Duration::from_secs(0),
            time_since_first_batch: Duration::from_secs(0),
            total_nodes: 0,
            nodes_since_movestart: 0,
            batches_since_movestart: 0,
            avg_depth: 0,
            edge_n: Vec::new(),
            win_found: false,
            may_resign: false,
            num_losing_edges: 0,
            time_usage_hint: TimeUsageHint::Normal,
        }
    }
}

/// Sent when search decides on the best move.
pub struct BestMoveInfo {
    best_move: Move,
    ponder: Move,
    is_black: Option<bool>,
}

struct Wdl {
    win: i32,
    draw: i32,
    loss: i32,
}

/// Sent during search
pub struct ThinkingInfo {
    depth: i32,
    seldepth: i32,
    time: Duration,
    nodes: usize,
    nps: usize,
    hashfull: usize,
    mate: Option<i32>,
    score: Option<i32>,
    wdl: Option<Wdl>,
    tb_hits: Option<usize>,
    pv: Vec<Move>,
    multipv: Option<usize>,
    comment: Option<String>,
}

