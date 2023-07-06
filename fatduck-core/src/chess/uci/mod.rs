use shakmaty::{Color, Move};

pub struct UciGoParams {
    wtime: Option<u64>,
    btime: Option<u64>,
    winc: Option<u64>,
    binc: Option<u64>,
    movestogo: Option<u64>,
    depth: Option<u64>,
    nodes: Option<u64>,
    movetime: Option<u64>,
    infinite: bool,
    searchmoves: Vec<String>,
    ponder: bool,
}

pub struct UciLoop;

/// Sends response(s) to host
impl UciLoop {
    pub fn send_response(&self, response: String) {
        todo!();
    }

    pub fn send_responses(&self, responses: Vec<String>) {
        todo!();
    }

    pub fn send_best_move(&self, best_move: Move) {
        todo!();
    }

    pub fn send_info(&self, info: Vec<ThinkingInfo>) {
        todo!();
    }

    pub fn send_id(&self) {
        todo!();
    }
}

/// Command handlers
impl UciLoop {
    pub fn cmd_uci(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_isready(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_setoption(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_ucinewgame(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_position(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_fen(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_go(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_stop(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_ponderhit(&self) -> Result<_> {
        todo!();
    }

    pub fn cmd_start(&self) -> Result<_> {
        todo!();
    }

    fn dispatch_cmd(&self, cmd: &str) -> Result<_> {
        todo!();
    }
}

enum GameResult {
    Decisive { winner: Color },
    Draw,
    Ongoing,
}

/// Sent when a single game is finished
pub struct GameInfo {
    game_result: GameResult,
    training_filename: String,
    initial_fen: String,
    moves: Vec<Move>,
    play_start_ply: usize,
    game_id: usize,
    is_black: Option<bool>,
    min_false_positive_threshold: Option<f32>,
}

/// Sent during and after a tournament is finished
pub struct TournamentInfo {
    finished: bool,
    results: [[u32; 2]; 3],
    move_count: usize,
    nodes_total: usize,
}
