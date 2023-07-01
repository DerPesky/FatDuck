use shakmaty::{Chess, Outcome, Position};

#[derive(Debug, Clone, Copy)]
pub enum Terminal {
    TwoFold,
    GameOver,
    Tablebase,
    NonTerminal,
}

#[derive(Default, Clone)]
pub struct GameState {
    position: Chess,
    /// How many half-moves since the position was repeated or 0.
    cycle_length: u8,
    // How many repetitions this position has had before.
    repetition_count: u8,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            position: Chess::default(),
            cycle_length: 0,
            repetition_count: 0,
        }
    }

    pub fn position(&self) -> &Chess {
        &self.position
    }

    pub fn compute_game_result(&self) -> Option<Outcome> {
        // `position.outcome()` does not consider 50-move rule or 3-fold repetition.
        if self.position.halfmoves() >= 100 || self.repetition_count >= 2 {
            return Some(Outcome::Draw);
        }

        self.position.outcome()
    }

    pub fn repetition_count(&self) -> u8 {
        self.repetition_count
    }
}
