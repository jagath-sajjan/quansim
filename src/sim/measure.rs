use rand::Rng;
use rand::thread_rng;

use super::state::QuantumState;

pub fn measure(state: &QuantumState) -> usize {
    let mut rng = thread_rng();
    let r: f64 = rng.r#gen();

    let mut cumulative = 0.0;

    for (i, amp) in state.amplitudes.iter().enumerate() {
        cumulative += amp.norm_sqr();

        if r < cumulative {
            return i;
        }
    }

    state.amplitudes.len() - 1
}

pub fn measure_many(state: &QuantumState, shots: usize) -> Vec<usize> {
    let mut results = Vec::with_capacity(shots);

    for _ in 0..shots {
        results.push(measure(state));
    }

    results
}
