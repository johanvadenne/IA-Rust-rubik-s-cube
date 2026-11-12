use std::path::Path;

use burn::{
    backend::{Autodiff, NdArray},
    module::Module,
    optim::{AdamConfig, GradientsParams, Optimizer},
    record::{CompactRecorder, Recorder},
    tensor::{backend::Backend, Tensor, TensorData},
};
use rand::Rng;

use crate::{
    cube::RubiksCube,
    encode::{encode_state, STATE_SIZE},
    net::ValueNet,
};

/// CPU backend used for inference.
pub type InferBackend = NdArray<f32, i32>;
/// Autodiff wrapper used during training.
pub type TrainBackend = Autodiff<InferBackend>;

#[derive(Debug, Clone)]
pub struct TrainConfig {
    pub max_depth: usize,
    pub samples_per_depth: usize,
    pub batch_size: usize,
    pub epochs_per_depth: usize,
    pub learning_rate: f64,
    pub artifact_dir: String,
}

impl Default for TrainConfig {
    fn default() -> Self {
        Self {
            max_depth: 8,
            samples_per_depth: 2_048,
            batch_size: 128,
            epochs_per_depth: 4,
            learning_rate: 1e-3,
            artifact_dir: "models".to_string(),
        }
    }
}

fn make_batch<B: Backend>(
    states: &[Vec<f32>],
    targets: &[f32],
    device: &B::Device,
) -> (Tensor<B, 2>, Tensor<B, 2>) {
    let batch = states.len();
    let flat: Vec<f32> = states.iter().flat_map(|s| s.iter().copied()).collect();
    let input = Tensor::<B, 1>::from_data(TensorData::new(flat, [batch * STATE_SIZE]), device)
        .reshape([batch, STATE_SIZE]);
    let target = Tensor::<B, 1>::from_data(TensorData::new(targets.to_vec(), [batch]), device)
        .reshape([batch, 1]);
    (input, target)
}

fn generate_samples(depth: usize, count: usize) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let mut states = Vec::with_capacity(count);
    let mut targets = Vec::with_capacity(count);

    for _ in 0..count {
        // Uniform depth in 0..=depth so the net also sees easy states.
        let d = if depth == 0 {
            0
        } else {
            rng.gen_range(0..=depth)
        };
        let mut cube = RubiksCube::new();
        if d > 0 {
            cube.scramble(d, &mut rng);
        }
        states.push(encode_state(&cube));
        targets.push(d as f32);
    }

    (states, targets)
}

/// Curriculum training: increase scramble depth and fit `V(s) ≈ d`.
pub fn train(config: TrainConfig) {
    std::fs::create_dir_all(&config.artifact_dir).expect("create artifact dir");

    let device = Default::default();
    let mut model = ValueNet::<TrainBackend>::new(&device);
    let mut optimizer = AdamConfig::new().init();

    println!(
        "Training ValueNet (curriculum 1..{}) → {}",
        config.max_depth, config.artifact_dir
    );

    for depth in 1..=config.max_depth {
        println!("=== Curriculum depth {depth} ===");
        let (states, targets) = generate_samples(depth, config.samples_per_depth);

        for epoch in 1..=config.epochs_per_depth {
            let mut order: Vec<usize> = (0..states.len()).collect();
            let mut rng = rand::thread_rng();
            // Fisher–Yates shuffle
            for i in (1..order.len()).rev() {
                let j = rng.gen_range(0..=i);
                order.swap(i, j);
            }

            let mut epoch_loss = 0.0f32;
            let mut batches = 0usize;

            for chunk in order.chunks(config.batch_size) {
                let batch_states: Vec<Vec<f32>> =
                    chunk.iter().map(|&i| states[i].clone()).collect();
                let batch_targets: Vec<f32> = chunk.iter().map(|&i| targets[i]).collect();
                let (input, target) =
                    make_batch::<TrainBackend>(&batch_states, &batch_targets, &device);

                let prediction = model.forward(input);
                let loss = prediction.sub(target).powf_scalar(2.0).mean();
                let loss_value = loss.clone().into_scalar();

                let grads = GradientsParams::from_grads(loss.backward(), &model);
                model = optimizer.step(config.learning_rate, model, grads);

                epoch_loss += loss_value;
                batches += 1;
            }

            let mean_loss = epoch_loss / batches.max(1) as f32;
            println!("  epoch {epoch}/{}  mse={mean_loss:.4}", config.epochs_per_depth);
        }

        let ckpt = format!("{}/value_d{depth}", config.artifact_dir);
        model
            .clone()
            .save_file(&ckpt, &CompactRecorder::new())
            .expect("save checkpoint");
        println!("  saved {ckpt}.mpk");
    }

    let final_path = format!("{}/value_v1", config.artifact_dir);
    model
        .save_file(&final_path, &CompactRecorder::new())
        .expect("save final model");
    println!("Training done. Final model: {final_path}.mpk");
}

/// Load a trained model for inference on the NdArray backend.
pub fn load_model(path: impl AsRef<Path>, device: &<InferBackend as Backend>::Device) -> ValueNet<InferBackend> {
    let path = path.as_ref();
    let record = CompactRecorder::new()
        .load(path.into(), device)
        .unwrap_or_else(|e| panic!("failed to load model from {}: {e}", path.display()));
    ValueNet::<InferBackend>::new(device).load_record(record)
}
