use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};
use rand::Rng;
use rubik_s_cube::{
    animate_solution, load_model, solve, train, Move, RubiksCube, SolveConfig, TrainConfig,
};

#[derive(Parser, Debug)]
#[command(name = "rubik_s_cube", about = "Solve a Rubik's cube with a learned value network")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Train the value network with a scramble-depth curriculum.
    Train {
        /// Maximum scramble depth in the curriculum.
        #[arg(long, default_value_t = 8)]
        max_depth: usize,
        /// Samples generated per curriculum depth.
        #[arg(long, default_value_t = 2048)]
        samples: usize,
        /// Epochs per curriculum depth.
        #[arg(long, default_value_t = 4)]
        epochs: usize,
        /// Mini-batch size.
        #[arg(long, default_value_t = 128)]
        batch_size: usize,
        /// Learning rate.
        #[arg(long, default_value_t = 1e-3)]
        lr: f64,
        /// Directory for checkpoints.
        #[arg(long, default_value = "models")]
        out: String,
    },
    /// Scramble then solve a cube using a trained model.
    Solve {
        /// Number of random scramble moves.
        #[arg(long, default_value_t = 5)]
        depth: usize,
        /// Path to model file without extension (e.g. models/value_v1).
        #[arg(long, default_value = "models/value_v1")]
        model: PathBuf,
        /// Max A* expansions.
        #[arg(long, default_value_t = 50_000)]
        max_nodes: usize,
        /// Heuristic weight for weighted A*.
        #[arg(long, default_value_t = 1.0)]
        weight: f32,
        /// Open a window and animate the solution.
        #[arg(long, default_value_t = false)]
        visual: bool,
        /// Delay between animated moves (ms).
        #[arg(long, default_value_t = 350)]
        delay_ms: u64,
    },
    /// Train briefly then run a short solve demo.
    Demo {
        /// Scramble depth for the demo solve.
        #[arg(long, default_value_t = 4)]
        depth: usize,
        /// Curriculum max depth for the quick training.
        #[arg(long, default_value_t = 5)]
        train_depth: usize,
        /// Skip the visual window.
        #[arg(long, default_value_t = false)]
        no_visual: bool,
        /// Delay between animated moves (ms).
        #[arg(long, default_value_t = 350)]
        delay_ms: u64,
    },
}

fn format_moves(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| m.label())
        .collect::<Vec<_>>()
        .join(" ")
}

fn run_solve(
    depth: usize,
    model_path: &PathBuf,
    max_nodes: usize,
    weight: f32,
    visual: bool,
    delay_ms: u64,
) {
    let device = Default::default();
    let model = load_model(model_path, &device);

    let mut rng = rand::thread_rng();
    let mut cube = RubiksCube::new();
    let scramble = cube.scramble(depth, &mut rng);

    println!("Scramble ({depth}): {}", format_moves(&scramble));
    println!("Start state:");
    cube.display();

    let start = Instant::now();
    let result = solve(
        &cube,
        &model,
        &device,
        SolveConfig {
            max_nodes,
            heuristic_weight: weight,
        },
    );
    let elapsed = start.elapsed();

    if result.solved {
        println!(
            "Solved in {} moves ({} nodes, {:?}): {}",
            result.moves.len(),
            result.nodes_expanded,
            elapsed,
            format_moves(&result.moves)
        );

        if visual {
            println!("Opening visual window (Esc to close)...");
            animate_solution(cube.clone(), &result.moves, delay_ms);
        }

        cube.apply_sequence(&result.moves);
        println!("Final state:");
        cube.display();
        println!("is_solved = {}", cube.is_solved());
    } else {
        println!(
            "No solution within {} nodes ({:?}).",
            result.nodes_expanded, elapsed
        );
        if visual {
            println!("Opening visual window with unsolved cube (Esc to close)...");
            rubik_s_cube::show_cube(&cube, "Rubik's Cube — unsolved");
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Train {
            max_depth,
            samples,
            epochs,
            batch_size,
            lr,
            out,
        } => {
            train(TrainConfig {
                max_depth,
                samples_per_depth: samples,
                batch_size,
                epochs_per_depth: epochs,
                learning_rate: lr,
                artifact_dir: out,
            });
        }
        Commands::Solve {
            depth,
            model,
            max_nodes,
            weight,
            visual,
            delay_ms,
        } => {
            run_solve(depth, &model, max_nodes, weight, visual, delay_ms);
        }
        Commands::Demo {
            depth,
            train_depth,
            no_visual,
            delay_ms,
        } => {
            let out = "models".to_string();
            train(TrainConfig {
                max_depth: train_depth,
                samples_per_depth: 1024,
                batch_size: 64,
                epochs_per_depth: 3,
                learning_rate: 1e-3,
                artifact_dir: out.clone(),
            });
            let model = PathBuf::from(format!("{out}/value_v1"));
            let d = if depth == 0 {
                1
            } else {
                rand::thread_rng().gen_range(1..=depth)
            };
            run_solve(d, &model, 20_000, 1.0, !no_visual, delay_ms);
        }
    }
}
