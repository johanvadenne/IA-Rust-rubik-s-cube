use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use burn::tensor::{backend::Backend, Tensor, TensorData};

use crate::{
    cube::{Move, RubiksCube, ALL_MOVES},
    encode::{encode_state, STATE_SIZE},
    net::ValueNet,
};

#[derive(Debug, Clone)]
pub struct SolveResult {
    pub moves: Vec<Move>,
    pub nodes_expanded: usize,
    pub solved: bool,
}

#[derive(Debug, Clone)]
pub struct SolveConfig {
    pub max_nodes: usize,
    /// Weight on the heuristic in `f = g + w * h` (weighted A*).
    pub heuristic_weight: f32,
}

impl Default for SolveConfig {
    fn default() -> Self {
        Self {
            max_nodes: 50_000,
            heuristic_weight: 1.0,
        }
    }
}

#[derive(Clone)]
struct Node {
    f: f32,
    g: u32,
    cube: RubiksCube,
    mv: Option<Move>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f && self.g == other.g
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap via reverse ordering on f, then prefer larger g.
        other
            .f
            .partial_cmp(&self.f)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.g.cmp(&other.g))
    }
}

/// Compact hash of the cube for closed-set keys.
fn cube_key(cube: &RubiksCube) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    cube.hash(&mut hasher);
    hasher.finish()
}

fn evaluate_batch<B: Backend>(
    model: &ValueNet<B>,
    cubes: &[RubiksCube],
    device: &B::Device,
) -> Vec<f32> {
    if cubes.is_empty() {
        return Vec::new();
    }
    let batch = cubes.len();
    let flat: Vec<f32> = cubes.iter().flat_map(encode_state).collect();
    let input = Tensor::<B, 1>::from_data(TensorData::new(flat, [batch * STATE_SIZE]), device)
        .reshape([batch, STATE_SIZE]);
    let values = model.forward(input);
    values
        .into_data()
        .to_vec::<f32>()
        .unwrap_or_else(|_| vec![0.0; batch])
        .into_iter()
        .map(|v| v.max(0.0))
        .collect()
}

/// Weighted A* guided by the value network.
pub fn solve<B: Backend>(
    start: &RubiksCube,
    model: &ValueNet<B>,
    device: &B::Device,
    config: SolveConfig,
) -> SolveResult {
    if start.is_solved() {
        return SolveResult {
            moves: Vec::new(),
            nodes_expanded: 0,
            solved: true,
        };
    }

    let start_h = evaluate_batch(model, &[start.clone()], device)[0];
    let start_key = cube_key(start);

    let mut open = BinaryHeap::new();
    open.push(Node {
        f: config.heuristic_weight * start_h,
        g: 0,
        cube: start.clone(),
        mv: None,
    });

    // key -> (g, parent_key, move_from_parent)
    let mut came_from: HashMap<u64, (u32, Option<u64>, Option<Move>)> = HashMap::new();
    came_from.insert(start_key, (0, None, None));

    let mut best_g: HashMap<u64, u32> = HashMap::new();
    best_g.insert(start_key, 0);

    let mut nodes_expanded = 0usize;

    while let Some(node) = open.pop() {
        let key = cube_key(&node.cube);
        if let Some(&bg) = best_g.get(&key) {
            if node.g > bg {
                continue;
            }
        }

        if node.cube.is_solved() {
            let moves = reconstruct_path(&came_from, key);
            return SolveResult {
                moves,
                nodes_expanded,
                solved: true,
            };
        }

        if nodes_expanded >= config.max_nodes {
            break;
        }
        nodes_expanded += 1;

        let mut children = Vec::with_capacity(ALL_MOVES.len());
        let mut child_cubes = Vec::with_capacity(ALL_MOVES.len());

        for &mv in &ALL_MOVES {
            if let Some(prev) = node.mv {
                if prev.same_face(mv) {
                    continue;
                }
            }
            let mut next = node.cube.clone();
            next.apply(mv);
            children.push(mv);
            child_cubes.push(next);
        }

        let heuristics = evaluate_batch(model, &child_cubes, device);

        for ((mv, next_cube), h) in children
            .into_iter()
            .zip(child_cubes.into_iter())
            .zip(heuristics.into_iter())
        {
            let next_g = node.g + 1;
            let next_key = cube_key(&next_cube);

            if let Some(&bg) = best_g.get(&next_key) {
                if next_g >= bg {
                    continue;
                }
            }

            best_g.insert(next_key, next_g);
            came_from.insert(next_key, (next_g, Some(key), Some(mv)));

            let f = next_g as f32 + config.heuristic_weight * h;
            open.push(Node {
                f,
                g: next_g,
                cube: next_cube,
                mv: Some(mv),
            });
        }
    }

    SolveResult {
        moves: Vec::new(),
        nodes_expanded,
        solved: false,
    }
}

fn reconstruct_path(
    came_from: &HashMap<u64, (u32, Option<u64>, Option<Move>)>,
    mut key: u64,
) -> Vec<Move> {
    let mut moves = Vec::new();
    while let Some((_g, parent, mv)) = came_from.get(&key) {
        if let Some(m) = mv {
            moves.push(*m);
        }
        match parent {
            Some(p) => key = *p,
            None => break,
        }
    }
    moves.reverse();
    moves
}
