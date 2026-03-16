use super::state::QuantumState;

pub fn probabilities(state: &QuantumState) -> Vec<f64> {
    state
        .amplitudes
        .iter()
        .map(|amp| amp.norm_sqr())
        .collect()
}
