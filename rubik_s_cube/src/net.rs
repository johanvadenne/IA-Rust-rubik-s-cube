use burn::{
    module::Module,
    nn::{Linear, LinearConfig, Relu},
    tensor::{backend::Backend, Tensor},
};

use crate::encode::STATE_SIZE;

/// MLP value network: estimates distance-to-solved.
#[derive(Module, Debug)]
pub struct ValueNet<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    linear3: Linear<B>,
    activation: Relu,
}

impl<B: Backend> ValueNet<B> {
    pub fn new(device: &B::Device) -> Self {
        Self {
            linear1: LinearConfig::new(STATE_SIZE, 512).init(device),
            linear2: LinearConfig::new(512, 256).init(device),
            linear3: LinearConfig::new(256, 1).init(device),
            activation: Relu::new(),
        }
    }

    /// Forward pass. Input shape `[batch, STATE_SIZE]`, output `[batch, 1]`.
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.activation.forward(self.linear1.forward(input));
        let x = self.activation.forward(self.linear2.forward(x));
        self.linear3.forward(x)
    }
}
