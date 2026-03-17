#![warn(clippy::all, clippy::pedantic)]

use burn::{
    Tensor,
    backend::{Autodiff, NdArray, ndarray::NdArrayDevice},
    module::Module,
    nn::{
        Linear, LinearConfig,
        loss::{MseLoss, Reduction},
    },
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::Backend,
    tensor::{activation::silu, backend::AutodiffBackend},
};

#[derive(Debug, Module)]
struct Model<B: Backend> {
    layers: Vec<Linear<B>>,
}

impl<B: Backend> Model<B> {
    fn new(device: &B::Device) -> Self {
        Self {
            layers: vec![
                LinearConfig::new(2, 4).init(device),
                LinearConfig::new(4, 1).init(device),
            ],
        }
    }

    fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.layers
            .iter()
            .fold(x, |i, layer| silu(layer.forward(i)))
    }
}

fn train<B: AutodiffBackend>(model: &Model<B>, device: &B::Device) -> Model<B> {
    let input = Tensor::from_floats([[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], device);
    let target = Tensor::from_floats([[0.0], [1.0], [1.0], [0.0]], device);

    let mut optim = AdamConfig::new().init();
    let mut model = model.clone();

    for _ in 0..100 {
        let pred = model.forward(input.clone());
        let loss = MseLoss::new().forward(pred.clone(), target.clone(), Reduction::Auto);

        let grads = loss.backward();
        let params = GradientsParams::from_grads(grads, &model);
        model = optim.step(0.1, model, params);
    }

    model
}

fn test<B: Backend>(model: &Model<B>, device: &B::Device) {
    let inputs = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let input = Tensor::from_floats(inputs, device);
    let output = model
        .forward(input)
        .into_data()
        .to_vec::<f32>()
        .expect("Failed to read output");
    for i in 0..inputs.len() {
        let a = inputs[i][0];
        let b = inputs[i][1];
        let c = if output[i] > 0.5 { 1.0 } else { 0.0 };
        println!("{a} xor {b} = {c}");
    }
}

fn main() {
    let device = NdArrayDevice::default();
    let model = Model::<Autodiff<NdArray>>::new(&device);
    let model = train(&model, &device);
    test(&model, &device);
}
