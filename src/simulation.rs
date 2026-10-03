// src/simulation.rs
use crate::coords::V3I128;

#[derive(Clone)]
pub struct BrainSpore {
    pub position: V3I128,
    pub neural_weights: Vec<f32>,
    pub energy: f32,
}

pub struct BrainSporeWorldBuffer {
    pub spores: Vec<BrainSpore>,
}

impl BrainSporeWorldBuffer {
    pub fn new() -> Self {
        Self { spores: Vec::new() }
    }

    pub fn spawn_spore(&mut self, position: V3I128, weights: Vec<f32>) {
        self.spores.push(BrainSpore {
            position,
            neural_weights: weights,
            energy: 100.0,
        });
    }

    pub fn simulate_tick(&mut self) {
        for spore in &mut self.spores {
            spore.energy -= 0.5;
            for w in &mut spore.neural_weights {
                *w = (*w * 0.99).clamp(-1.0, 1.0);
            }
        }
    }
}
