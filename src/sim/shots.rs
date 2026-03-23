use std::collections::HashMap;
use rayon::prelude::*;
use super::state::QuantumState;
use super::measure::measure;

pub fn run_shots(state: &QuantumState, shots: usize) -> HashMap<usize, usize> {
    (0..shots)
        .into_par_iter()
        .map(|_| measure(state))
        .fold(
            || HashMap::<usize, usize>::new(),
            |mut acc, r| {
                *acc.entry(r).or_insert(0) += 1;
                acc
            },
        )
        .reduce(
            || HashMap::<usize, usize>::new(),
            |mut a, b| {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
                a
            },
        )
}
