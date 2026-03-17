use std::collections::HashMap;
use super::state::QuantumState;
use super::measure::measure;

pub fn run_shots(state: &QuantumState, shots: usize) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();

    for _ in 0..shots {
        let result = measure(state);
        *counts.entry(result).or_insert(0) += 1;
    }

    counts
}
