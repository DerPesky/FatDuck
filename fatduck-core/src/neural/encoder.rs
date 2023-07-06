use crate::{
    chess::GameState,
    neural::network::{
        InputPlaneStack, AUX_PLANE_BASE, MOVE_HISTORY, NUM_INPUT_PLANES, PLANES_PER_BOARD,
    },
    pblczero,
};

use shakmaty::{
    Bitboard, Board, Castles, CastlingMode, CastlingSide, Chess, Color, EnPassantMode, Position,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FillEmptyHistory {
    No,
    FenOnly,
    Always,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BoardTransform {
    // Horizontal mirror
    Flip = 1,
    // Vertical mirror
    Mirror = 2,
    // Diagonal transpose a1 to h8
    Transpose = 4,
}

struct InputStackAugmenter;
impl InputStackAugmenter {
    // 0, 1, 2, 3, 4, 5, 7, 12
    pub fn castling(stack: &mut InputPlaneStack<NUM_INPUT_PLANES>, position: &Chess) {
        let (us, them) = match position.turn() {
            Color::White => (Color::White, Color::Black),
            Color::Black => (Color::Black, Color::White),
        };

        if position.castles().has(us, CastlingSide::QueenSide) {
            stack.planes_mut()[AUX_PLANE_BASE].set_mask_max();
        }

        if position.castles().has(us, CastlingSide::KingSide) {
            stack.planes_mut()[AUX_PLANE_BASE + 1].set_mask_max();
        }

        if position.castles().has(them, CastlingSide::QueenSide) {
            stack.planes_mut()[AUX_PLANE_BASE + 2].set_mask_max();
        }

        if position.castles().has(them, CastlingSide::KingSide) {
            stack.planes_mut()[AUX_PLANE_BASE + 3].set_mask_max();
        }
    }

    pub fn help_find_edges(stack: &mut InputPlaneStack<NUM_INPUT_PLANES>) {
        stack.planes_mut()[AUX_PLANE_BASE + 7].set_mask_max();
    }

    pub fn two_fold_checked(stack: &mut InputPlaneStack<NUM_INPUT_PLANES>, state: &GameState) {
        if state.repetition_count() >= 1 {
            stack.planes_mut()[AUX_PLANE_BASE + 12].set_mask_max();
        }
    }

    pub fn fifty_move_rule(stack: &mut InputPlaneStack<NUM_INPUT_PLANES>, state: &GameState) {
        stack.planes_mut()[AUX_PLANE_BASE + 5].fill(state.position().halfmoves() as f32);
    }

    pub fn black_turn(stack: &mut InputPlaneStack<NUM_INPUT_PLANES>) {
        stack.planes_mut()[AUX_PLANE_BASE + 4].set_mask_max()
    }

    pub fn piece_planes(
        stack: &mut InputPlaneStack<NUM_INPUT_PLANES>,
        board: &Board,
        us_them_colors: (Color, Color),
        offset: usize,
    ) {
        let (us, them) = us_them_colors;

        let our_pieces = board.by_color(us);
        *stack.planes_mut()[offset].mask_mut() = our_pieces.intersect(board.pawns()).into();
        *stack.planes_mut()[offset + 1].mask_mut() = our_pieces.intersect(board.knights()).into();
        *stack.planes_mut()[offset + 2].mask_mut() = our_pieces.intersect(board.bishops()).into();
        *stack.planes_mut()[offset + 3].mask_mut() = our_pieces.intersect(board.rooks()).into();
        *stack.planes_mut()[offset + 4].mask_mut() = our_pieces.intersect(board.queens()).into();
        *stack.planes_mut()[offset + 5].mask_mut() = our_pieces.intersect(board.kings()).into();

        let their_pieces = board.by_color(them);
        *stack.planes_mut()[offset + 6].mask_mut() = their_pieces.intersect(board.pawns()).into();
        *stack.planes_mut()[offset + 7].mask_mut() = their_pieces.intersect(board.knights()).into();
        *stack.planes_mut()[offset + 8].mask_mut() = their_pieces.intersect(board.bishops()).into();
        *stack.planes_mut()[offset + 9].mask_mut() = their_pieces.intersect(board.rooks()).into();
        *stack.planes_mut()[offset + 10].mask_mut() = their_pieces.intersect(board.queens()).into();
        *stack.planes_mut()[offset + 11].mask_mut() = their_pieces.intersect(board.kings()).into();
    }

    pub fn transform_masks(
        stack: &mut InputPlaneStack<NUM_INPUT_PLANES>,
        transform: Option<BoardTransform>,
    ) {
        if transform.is_none() {
            return;
        }

        for plane in stack.planes_mut().iter_mut().take(AUX_PLANE_BASE + 4 + 1) {
            if plane.mask() == 0 || plane.mask() == u64::MAX {
                continue;
            }

            let mut mask_bboard = Bitboard::from(plane.mask());

            match transform {
                Some(BoardTransform::Flip) => mask_bboard = mask_bboard.flip_horizontal(),
                Some(BoardTransform::Mirror) => mask_bboard = mask_bboard.flip_vertical(),
                Some(BoardTransform::Transpose) => mask_bboard = mask_bboard.flip_diagonal(),
                // SAFETY: None is early returned above.
                None => unsafe { std::hint::unreachable_unchecked() },
            }

            *plane.mask_mut() = mask_bboard.into();
        }
    }
}

impl<const N: usize> InputPlaneStack<N> {
    pub fn encode_position_for_nn(
        // input_format: pblczero::network_format::InputFormat,
        history: &[GameState],
        history_planes: usize,
        fill_setting: FillEmptyHistory,
        transform_out: Option<&mut BoardTransform>,
    ) -> InputPlaneStack<NUM_INPUT_PLANES> {
        let mut result = InputPlaneStack::new();
        let current_state = history.last().expect("History is empty");

        // match input_format {
        // pblczero::network_format::InputFormat::InputClassical112Plane => {
        InputStackAugmenter::castling(&mut result, current_state.position());
        // }
        // _ => panic!("Unsupported input format: {input_format:?}"),
        // };

        if current_state.position().turn().is_black() {
            InputStackAugmenter::black_turn(&mut result);
        }

        InputStackAugmenter::fifty_move_rule(&mut result, current_state);
        InputStackAugmenter::help_find_edges(&mut result);

        let mut flip = false;
        let mut history_idx = (history.len() as i32) - 1;

        for i in 0..std::cmp::min(history_planes, MOVE_HISTORY) {
            // SAFETY: current_state will already panic on unwrap if history is empty
            let state = unsafe { history.get_unchecked(history.len().saturating_sub(1)) };

            if flip {
                state.position().board().rotate_180();
            }

            let mut board = state.position().board();

            // Castling changes can't be repeated, so we can stop early.
            if state.position().castles().castling_rights()
                == Castles::empty(CastlingMode::Standard).castling_rights()
            {
                break;
            }

            // En passant's cant be repeated, but we do need to always send the current position.
            if history_idx != (history.len() as i32 - 1)
                && state.position().legal_ep_square().is_some()
            {
                break;
            }

            // If en passant is possible we know the previous move.
            if fill_setting == FillEmptyHistory::No
                && (history_idx < -1
                    || history_idx == -1 && current_state.position().legal_ep_square().is_none())
            {
                break;
            }

            // Board may be flipped so compare with the original board
            if history_idx < 0
                && fill_setting == FillEmptyHistory::FenOnly
                && *state.position() == Chess::new()
            {
                break;
            }

            let (us, them) = match state.position().turn() {
                Color::White => (Color::White, Color::Black),
                Color::Black => (Color::Black, Color::White),
            };

            let base_offset = i * PLANES_PER_BOARD;
            InputStackAugmenter::piece_planes(&mut result, &board, (us, them), base_offset);
            InputStackAugmenter::two_fold_checked(&mut result, state);

            if let (Some(ep_square), true) = (
                state.position().ep_square(EnPassantMode::Legal),
                history_idx < 0,
            ) {
                let idx = u64::from(ep_square).trailing_zeros();

                if idx < 8 {
                    *result.planes_mut()[base_offset].mask_mut() +=
                        0x100_u64.wrapping_sub(0x1_0000_0000_u64) << idx;
                } else {
                    *result.planes_mut()[base_offset + 6].mask_mut() +=
                        (0x1_0000_0000_0000_u64 - 0x1_0000_0000_u64) << (idx.wrapping_sub(56));
                }
            }

            if history_idx > 0 {
                flip = !flip;
            }

            if state.position().halfmoves() == 0 {
                break;
            }

            history_idx -= 1;
        }

        let mut transform = None;

        if transform.is_some() {
            InputStackAugmenter::transform_masks(&mut result, transform);
        }

        if let Some(t_out) = transform_out {
            *t_out = transform.unwrap();
        }

        result
    }
}
