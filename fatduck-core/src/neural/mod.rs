use crate::chess::GameState;
use shakmaty::Move;

mod backends;
mod encoder;
mod loader;
mod network;

// Only used to retrieve a quantitative evaluation of a GameState. Maybe it should return an order
// of moves to search? Or something that is more generic and not raw numbers.
// Idea is that someone can implement their own Evaluator that returns a score based off whatever
// algo they want. They can return a naive piece total as the score from the state or some more
// elaborate thing.
pub trait NNEvaluator: Default + Copy + Clone + Sync + Send {
    // the inputs needed to compute a score. (i.e wdl, nodes, piece type, player color etc.)
    type Input;

    // Impl's will probably call eval_by_info in eval_state (extract info from state).
    fn eval_by_info(&self, info: Self::Input) -> f32;
    fn eval_state(&self, state: &GameState, moves: &[Move]) -> f32;
}
