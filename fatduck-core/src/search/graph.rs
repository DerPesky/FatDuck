use crate::chess::GameState;
use petgraph::prelude::DiGraphMap;
use shakmaty::Move;

// TODO: Make more compact, don't store complete state (also for hash etc.)
pub struct GameGraph(pub DiGraphMap<u8, u8>);

impl GameGraph {
    pub fn new(root: GameState) -> Self {
        GameGraph(DiGraphMap::new())
    }

    // Compute new board state from given move and n - 1 state
    pub fn add_visited_node(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }

    pub fn add_unvisited_node(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }

    pub fn add_unvisited_edge(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }

    pub fn add_visited_edge(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }

    pub fn backprop_hot_path(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }

    pub fn backprop_brute(&mut self, move_: Move, current_state: &GameState) {
        todo!()
    }
}
