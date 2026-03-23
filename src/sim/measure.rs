use rand::Rng;
use rand::thread_rng;
use rayon::prelude::*;
use num_complex::Complex64;
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
    (0..shots)
        .into_par_iter()
        .map(|_| measure(state))
        .collect()
}

pub fn measure_qubit(state: &mut QuantumState, qubit: usize) -> u8 {
    let mut prob_one = 0.0;

    for (i, amp) in state.amplitudes.iter().enumerate() {
        if (i >> qubit) & 1 == 1 {
            prob_one += amp.norm_sqr();
        }
    }

    let mut rng = thread_rng();
    let r: f64 = rng.r#gen();
    let outcome = if r < prob_one { 1u8 } else { 0u8 };

    let norm = if outcome == 1 {
        prob_one.sqrt()
    } else {
        (1.0 - prob_one).sqrt()
    };

    for (i, amp) in state.amplitudes.iter_mut().enumerate() {
        if (i >> qubit) & 1 != outcome as usize {
            *amp = Complex64::new(0.0, 0.0);
        } else {
            *amp /= norm;
        }
    }

    outcome
}
