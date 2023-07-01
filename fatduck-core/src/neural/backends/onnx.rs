use crate::{
    neural::network::{
        InputStack, Network, NetworkCapabilities, NetworkComputation, NUM_INPUT_PLANES,
    },
    pblczero::network_format::{InputFormat, MovesLeftFormat},
};
use ort::OrtResult;
use std::{path::Path, sync::Arc};

pub struct OnnxNetwork {
    capabilities: NetworkCapabilities,
    environment: Arc<ort::Environment>,
    session: ort::Session,
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
    fn add_input(&mut self, planes: InputStack<NUM_INPUT_PLANES>) {
        todo!()
    }

    fn compute_blocking(&self) {
        todo!()
    }

    fn batch_size(&self) -> usize {
        todo!()
    }

    fn q_val(&self, sample: usize) -> f32 {
        todo!()
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
