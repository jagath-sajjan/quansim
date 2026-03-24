use rand::Rng;
use rand::thread_rng;
use super::state::QuantumState;
use super::gates::{x, rz, normalize};

pub struct NoiseModel {
    pub depolarizing: f64,
    pub bit_flip: f64,
    pub phase_flip: f64,
}

impl NoiseModel {
    pub fn new(depolarizing: f64, bit_flip: f64, phase_flip: f64) -> Self {
        Self { depolarizing, bit_flip, phase_flip }
    }

    pub fn ideal() -> Self {
        Self { depolarizing: 0.0, bit_flip: 0.0, phase_flip: 0.0 }
    }

    pub fn apply(&self, state: &mut QuantumState) {
        let mut rng = thread_rng();

        for q in 0..state.qubits {
            if self.bit_flip > 0.0 && rng.r#gen::<f64>() < self.bit_flip {
                x(state, q);
            }

            if self.phase_flip > 0.0 && rng.r#gen::<f64>() < self.phase_flip {
                rz(state, q, std::f64::consts::PI);
            }

            if self.depolarizing > 0.0 && rng.r#gen::<f64>() < self.depolarizing {
                let n = state.amplitudes.len();
                let step = 1 << q;
                for i in (0..n).step_by(step * 2) {
                    for j in 0..step {
                        state.amplitudes[i + j] *= (1.0 - self.depolarizing).sqrt();
                        state.amplitudes[i + j + step] *= (1.0 - self.depolarizing).sqrt();
                    }
                }
                normalize(state);
            }
        }
    }
}
