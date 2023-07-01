use crate::pblczero;
use shakmaty::Bitboard;
use std::fmt;

pub struct NetworkCapabilities {
    input_format: pblczero::network_format::InputFormat,
    moves_left_format: pblczero::network_format::MovesLeftFormat,
}

impl NetworkCapabilities {
    pub fn new(
        input_format: pblczero::network_format::InputFormat,
        moves_left_format: pblczero::network_format::MovesLeftFormat,
    ) -> Self {
        Self {
            input_format,
            moves_left_format,
        }
    }
}

pub trait Network {
    fn capabilities(&self) -> &NetworkCapabilities;
    fn new_computation(&self) -> Box<dyn NetworkComputation>;
}

pub trait NetworkComputation {
    fn add_input(&mut self, planes: InputStack<NUM_INPUT_PLANES>);
    fn compute_blocking(&self);
    fn batch_size(&self) -> usize;
    fn q_val(&self, sample: usize) -> f32;
    fn d_val(&self, sample: usize) -> f32;
    fn p_val(&self, sample: usize, move_id: usize) -> f32;
    fn m_val(&self, sample: usize) -> f32;
}

pub const MOVE_HISTORY: usize = 8;
pub const PLANES_PER_BOARD: usize = 13;
pub const AUX_PLANE_BASE: usize = PLANES_PER_BOARD * MOVE_HISTORY;
/// Number of input planes per stack of input
pub const NUM_INPUT_PLANES: usize = 112;

#[derive(Default, Clone, Copy)]
pub struct InputPlane {
    mask: u64,
    value: f32,
}

impl fmt::Debug for InputPlane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InputPlane {{ mask: {:?}, value: {} }}",
            Bitboard::from(self.mask),
            self.value
        )
    }
}

impl InputPlane {
    pub fn set_mask_max(&mut self) {
        self.mask = u64::MAX;
    }

    pub fn fill(&mut self, value: f32) {
        self.mask = u64::MAX;
        self.value = value;
    }

    pub fn mask(&self) -> u64 {
        self.mask
    }

    pub fn mask_mut(&mut self) -> &mut u64 {
        &mut self.mask
    }
}

// A stack of input planes
#[derive(Debug)]
pub struct InputStack<const N: usize>([InputPlane; N]);

impl<const N: usize> InputStack<N> {
    pub fn new() -> Self {
        Self([InputPlane::default(); N])
    }

    pub fn planes(&self) -> &[InputPlane] {
        &self.0
    }

    pub fn planes_mut(&mut self) -> &mut [InputPlane] {
        &mut self.0
    }
}
