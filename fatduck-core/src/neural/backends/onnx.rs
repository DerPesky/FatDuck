use crate::{
    neural::network::{
        InputPlaneStack, Network, NetworkCapabilities, NetworkComputation, NUM_INPUT_PLANES,
    },
    pblczero::network_format::{InputFormat, MovesLeftFormat},
};

use ort::{tensor::DynOrtTensor, OrtResult};
use std::{path::Path, sync::Arc};

pub struct OnnxNetwork {
    capabilities: NetworkCapabilities,
    environment: Arc<ort::Environment>,
    session: ort::Session,
    policy_head: Option<usize>,
    value_head: Option<usize>,
    wdl_head: Option<usize>,
    mlh_head: Option<usize>,
}

impl OnnxNetwork {
    // Create a new ONNX network from the given file
    pub fn from_file(filepath: &Path) -> OrtResult<Self> {
        let environment = Arc::new(
            ort::Environment::builder()
                .with_name("FatDuck")
                .with_execution_providers([ort::ExecutionProvider::cuda()])
                .build()?,
        );

        let session = ort::SessionBuilder::new(&environment)?
            .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
            .with_intra_threads(2)?
            .with_model_from_file(filepath)?;

        // TODO: Extract capabilities from ONNX file
        let input_format = InputFormat::InputClassical112Plane;
        let mlh_format = MovesLeftFormat::MovesLeftV1;

        Ok(Self {
            capabilities: NetworkCapabilities::new(input_format, mlh_format),
            environment,
            session,
        })
    }
}

impl Network for OnnxNetwork {
    fn capabilities(&self) -> &NetworkCapabilities {
        &self.capabilities
    }

    fn new_computation(&self) -> Box<dyn NetworkComputation> {
        unimplemented!();
    }
}

pub struct OnnxComputation<'a> {
    network: &'a OnnxNetwork,
}

impl NetworkComputation for OnnxComputation<'_> {
    fn add_input(&mut self, encoded_planes: InputPlaneStack<NUM_INPUT_PLANES>) {}

    fn compute_blocking(
        &self,
        encoded_planes: InputPlaneStack<NUM_INPUT_PLANES>,
    ) -> OrtResult<Vec<DynOrtTensor<'_, ndarray::IxDyn>>> {
        self.network.session.run(encoded_planes)
    }

    fn batch_size(&self) -> usize {
        todo!()
    }

    fn q_val(&self, sample: usize) -> f32 {
        if self.network.wdl
    }

    fn d_val(&self, sample: usize) -> f32 {
        todo!()
    }

    fn p_val(&self, sample: usize, move_id: usize) -> f32 {
        todo!()
    }

    fn m_val(&self, sample: usize) -> f32 {
        todo!()
    }
}
